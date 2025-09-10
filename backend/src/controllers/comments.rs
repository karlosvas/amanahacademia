use {
    crate::{
        controllers::users::get_user_data_db,
        models::{
            comments::{Comment, CommentRequest, FirebaseCommentResponse},
            firebase::UserAuthentication,
            user::UserDB,
        },
        services::firebase::handle_firebase_response,
        state::AppState,
    },
    axum::{
        Extension, Json, debug_handler, extract::Path, extract::State, http::StatusCode,
        response::IntoResponse,
    },
    serde_json::json,
    std::{collections::HashMap, sync::Arc},
};

// Crear un comentario
#[debug_handler]
pub async fn add_comment(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
    Json(comment): Json<CommentRequest>,
) -> impl IntoResponse {
    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/comments.json?auth={}",
        state.firebase.firebase_project_id, id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let new_comment: Comment = Comment {
        uid: user_claims.sub.clone(),
        name: comment.name.clone(),
        fecha: comment.fecha.clone(),
        content: comment.content.clone(),
        url_img: comment.url_img.clone(),
        like: 0,
        reply: Vec::new(),
        users_liked: Vec::new(),
    };

    // Enviamos el comentario a la base de datos para su creación
    match state
        .firebase_client
        .post(&url_firebase_db)
        .json(&new_comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<FirebaseCommentResponse>(response).await {
            Ok(data_response) => (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "comment_id": data_response.name,
                    "message": "Comments fetched successfully"
                })),
            )
                .into_response(),
            Err((status, error)) => return (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to add comment"})),
        )
            .into_response(),
    }
}

// Obtener todos los comentarios
#[debug_handler]
pub async fn get_all_comments(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Nos aseguramos que quien accede a estos datos es un administrador
    let actual_user_db: UserDB = match get_user_data_db(&user_claims, &id_token, &state).await {
        Some(user_data) => user_data,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            )
                .into_response();
        }
    };

    // Solo podemos obtener todos los usuarios si es administrador
    if actual_user_db.role != Some("admin".to_string()) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "You do not have permission to access this resource" })),
        )
            .into_response();
    }

    // URL para obtener todos los comentarios de la DB
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/comments.json?auth={}",
        state.firebase.firebase_project_id, id_token
    );

    // Realizamos la petición a la base de datos
    match state.firebase_client.get(&url_firebase_db).send().await {
        Ok(response) => {
            let response_text = response.text().await.unwrap_or_default();

            if response_text == "null" || response_text.is_empty() {
                // No hay comentarios, devolver HashMap vacío
                let empty_comments: HashMap<String, Comment> = HashMap::new();
                return (StatusCode::OK, Json(json!(empty_comments))).into_response();
            }

            match serde_json::from_str::<HashMap<String, Comment>>(&response_text) {
                Ok(comments) => (StatusCode::OK, Json(json!(comments))).into_response(),
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Error parsing Firebase response", "details": e.to_string()})),
                    ).into_response()
                }
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch comments"})),
        )
            .into_response(),
    }
}

// Eliminar comentario
#[debug_handler]
pub async fn delete_comment(
    Path(comment_id): Path<String>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos el ID del comentario a eliminar
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/comments/{}.json?auth={}",
        state.firebase.firebase_project_id, comment_id, id_token
    );

    // Verificar si el comentario existe
    match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Comment not found"})),
            )
                .into_response();
        }
    };

    // Intentamos eliminar el comentario
    match state.firebase_client.delete(&url_firebase_db).send().await {
        Ok(response) => {
            if response.status().is_success() {
                (StatusCode::NO_CONTENT, Json(json!({}))).into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to delete comment"})),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to delete comment"})),
        )
            .into_response(),
    }
}

// Obtener un comentario
async fn get_comment_data(
    comment_id: &str,
    id_token: &str,
    state: &Arc<AppState>,
) -> Option<Comment> {
    // URL de Firebase Realtime Database para obtener los datos del comentario
    let url_firebase_db: String = format!(
        "https://{}.firebasedatabase.app/comments/{}.json?auth={}",
        state.firebase.firebase_project_id, comment_id, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    match state.firebase_client.get(url_firebase_db).send().await {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(comment) => Some(comment),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

// Añadir o quitar el like
#[debug_handler]
pub async fn toggle_like(
    Path(comment_id): Path<String>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos el comentario
    let comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Comment not found"})),
            )
                .into_response();
        }
    };

    // Actualizamos el estado del "like"
    let url_firebase_db_like: String = format!(
        "https://{}.firebasedatabase.app/comments/{}.json?auth={}",
        state.firebase.firebase_project_id, comment_id, id_token
    );

    // Comprobamos si ya le habiamos dado like
    let is_liked: bool = comment.users_liked.contains(&comment.uid);
    let new_comment: Comment = Comment {
        like: if is_liked {
            comment.like.saturating_sub(1)
        } else {
            comment.like + 1
        },
        users_liked: if is_liked {
            let mut users_liked: Vec<String> = comment.users_liked.clone();
            users_liked.retain(|uid| uid != &comment.uid);
            users_liked
        } else {
            let mut users_liked: Vec<String> = comment.users_liked.clone();
            users_liked.push(comment.uid.clone());
            users_liked
        },
        ..comment
    };

    // Actualizamos el comentario
    match state
        .firebase_client
        .put(&url_firebase_db_like)
        .json(&new_comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(comment) => (StatusCode::OK, Json(json!({"comment": comment}))).into_response(),
            Err((status, error)) => (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update comment"})),
        )
            .into_response(),
    }
}

#[debug_handler]
pub async fn add_reply(
    Path(comment_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    Json(reply_comment): Json<CommentRequest>,
) -> impl IntoResponse {
    // Obtenemos el comentario al que se va a responder
    let comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"message": "Comment not found"})),
            )
                .into_response();
        }
    };

    // URL para obtener el comentario
    let url_firebase_db_comment: String = format!(
        "https://{}.firebasedatabase.app/comments/{}.json?auth={}",
        state.firebase.firebase_project_id, comment_id, id_token
    );

    // Creamos el nuevo comentario
    let new_comment: Comment = Comment {
        reply: {
            let mut replies: Vec<Comment> = comment.reply.clone();
            replies.push(Comment {
                uid: user_claims.sub.clone(),
                name: reply_comment.name.clone(),
                fecha: reply_comment.fecha.clone(),
                content: reply_comment.content.clone(),
                url_img: reply_comment.url_img.clone(),
                like: 0,
                reply: Vec::new(),
                users_liked: Vec::new(),
            });
            replies
        },
        ..comment
    };

    // Añadir el comentario a la base de datos
    match state
        .firebase_client
        .put(&url_firebase_db_comment)
        .json(&new_comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(comment) => (StatusCode::CREATED, Json(json!({"comment": comment}))).into_response(),
            Err((status, error)) => (status, Json(error)).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to add comment"})),
        )
            .into_response(),
    }
}
