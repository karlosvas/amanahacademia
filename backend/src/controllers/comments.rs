use {
    crate::{
        models::{comments::Comment, firebase::UserAuthentication, response::ResponseAPI},
        services::firebase::handle_firebase_response,
        state::AppState,
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    std::{collections::HashMap, sync::Arc},
    tracing::instrument,
};

// Crear un comentario
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims, comment),
    fields(
        user_id = %user_claims.user_id,
        comment_content_length = %comment.content.len(),
        operation = "add_comment"
    ))]
pub async fn add_comment(
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
    Json(comment): Json<Comment>,
) -> impl IntoResponse {
    // URL de para crear usuario en la DB
    let url_firebase_db: String = format!(
        "{}/comments.json?auth={}",
        state.firebase.firebase_database_url, id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let new_comment: Comment = Comment {
        author_uid: Some(user_claims.user_id),
        name: comment.name,
        timestamp: comment.timestamp,
        content: comment.content,
        url_img: comment.url_img,
        stars: comment.stars,
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
        Ok(response) => match handle_firebase_response::<HashMap<String, String>>(response).await {
            Ok(data_response) => (
                StatusCode::CREATED,
                Json(ResponseAPI::<HashMap<String, String>>::success(
                    "Comment created successfully".to_string(),
                    data_response,
                )),
            )
                .into_response(),
            Err((status, error)) => {
                return (status, Json(ResponseAPI::<()>::error(error))).into_response();
            }
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to add comment".to_string(),
            )),
        )
            .into_response(),
    }
}

// Obtener todos los comentarios
#[debug_handler]
#[instrument(skip(state), fields(operation = "get_all_comments"))]
pub async fn get_all_comments(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // URL para obtener todos los comentarios de la DB
    let url_firebase_db: String = format!("{}/comments.json", state.firebase.firebase_database_url);

    // Realizamos la petición a la base de datos
    match state.firebase_client.get(&url_firebase_db).send().await {
        Ok(response) => {
            let response_text: String = response.text().await.unwrap_or_default();

            if response_text == "null" || response_text.is_empty() {
                // No hay comentarios, devolver HashMap vacío
                return (
                    StatusCode::OK,
                    Json(ResponseAPI::<HashMap<String, Comment>>::success(
                        "No comments found".to_string(),
                        HashMap::new(),
                    )),
                )
                    .into_response();
            }

            match serde_json::from_str::<HashMap<String, Comment>>(&response_text) {
                Ok(comments) => {
                    let hidden_comments: HashMap<String, Comment> = comments
                        .into_iter()
                        .map(|(id, mut c)| {
                            c.author_uid = None; // Ocultamos el uid
                            (id, c)
                        })
                        .collect();
                    (
                        StatusCode::OK,
                        Json(ResponseAPI::<HashMap<String, Comment>>::success(
                            "Comments fetched successfully".to_string(),
                            hidden_comments,
                        )),
                    )
                        .into_response()
                }
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Error parsing Firebase response".to_string(),
                    )),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to fetch comments".to_string(),
            )),
        )
            .into_response(),
    }
}

// Eliminar comentario
#[debug_handler]
#[instrument(
    skip(state, id_token),
    fields(
        comment_id = %comment_id,
        operation = "delete_comment"
    )
)]
pub async fn delete_comment(
    Path(comment_id): Path<String>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos el ID del comentario a eliminar
    let url_firebase_db: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase.firebase_database_url, comment_id, id_token
    );

    // Verificar si el comentario existe
    match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Intentamos eliminar el comentario
    match state.firebase_client.delete(&url_firebase_db).send().await {
        Ok(response) => {
            if response.status().is_success() {
                (
                    StatusCode::NO_CONTENT,
                    Json(ResponseAPI::<()>::success_no_data()),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Failed to delete comment".to_string(),
                    )),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to delete comment".to_string(),
            )),
        )
            .into_response(),
    }
}

// Añadir o quitar el like
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims),
    fields(
        comment_id = %comment_id,
        user_id = %user_claims.user_id,
        operation = "toggle_like"
    )
)]
pub async fn toggle_like(
    Path(comment_id): Path<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos el comentario
    let comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Actualizamos el estado del "like"
    let url_firebase_db_like: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase.firebase_database_url, comment_id, id_token
    );

    // Comprobamos si ya le habiamos dado like
    let is_liked: bool = comment.users_liked.contains(&user_claims.user_id);

    let new_comment: Comment = Comment {
        like: if is_liked {
            comment.like.saturating_sub(1)
        } else {
            comment.like + 1
        },
        users_liked: {
            if is_liked {
                comment
                    .users_liked
                    .into_iter()
                    .filter(|uid| uid != &user_claims.user_id)
                    .collect()
            } else {
                let mut users = comment.users_liked.clone();
                users.push(user_claims.user_id.clone());
                users
            }
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
            Ok(comment) => {
                (
                    StatusCode::OK,
                    Json(ResponseAPI::<Comment>::success(
                        "Comment updated successfully".to_string(),
                        Comment {
                            author_uid: None,
                            ..comment
                        },
                    )),
                )
            }
            .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to update comment".to_string(),
            )),
        )
            .into_response(),
    }
}

// Añadir respuesta a un comentario
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims, reply_comment),
    fields(
        comment_id = %comment_id,
        user_id = %user_claims.user_id,
        reply_content_length = %reply_comment.content.len(),
        operation = "add_reply"
    )
)]
pub async fn add_reply(
    Path(comment_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    Json(reply_comment): Json<Comment>,
) -> impl IntoResponse {
    // Obtenemos el comentario al que se va a responder
    let comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // URL para obtener el comentario
    let url_firebase_db_comment: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase.firebase_database_url, comment_id, id_token
    );

    // Creamos el nuevo comentario
    let new_comment: Comment = Comment {
        reply: {
            let mut replies: Vec<Comment> = comment.reply.clone();
            replies.push(Comment {
                author_uid: Some(user_claims.user_id),
                name: reply_comment.name,
                timestamp: reply_comment.timestamp,
                content: reply_comment.content,
                url_img: reply_comment.url_img,
                stars: reply_comment.stars,
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
            Ok(comment) => (
                StatusCode::CREATED,
                Json(ResponseAPI::<Comment>::success(
                    "Comment added successfully".to_string(),
                    Comment {
                        author_uid: None,
                        ..comment
                    },
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to add comment".to_string(),
            )),
        )
            .into_response(),
    }
}

// Servicio para obtener un comentario de la base de datos
#[instrument(
    skip(state, id_token),
    fields(
        comment_id = %comment_id,
        operation = "get_comment_data"
    )
)]
async fn get_comment_data(
    comment_id: &str,
    id_token: &str,
    state: &Arc<AppState>,
) -> Option<Comment> {
    // URL de Firebase Realtime Database para obtener los datos del comentario
    let url_firebase_db: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase.firebase_database_url, comment_id, id_token
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
