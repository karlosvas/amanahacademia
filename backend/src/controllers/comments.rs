use {
    crate::{
        models::{
            comments::{Comment, ReplyComment, UpdateComment},
            firebase::UserAuthentication,
            response::ResponseAPI,
            state::AppState,
        },
        services::firebase::handle_firebase_response,
    },
    axum::{
        Extension, Json, debug_handler,
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    },
    chrono::Utc,
    std::{collections::HashMap, sync::Arc},
    tracing::instrument,
    uuid::Uuid,
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
        state.firebase_options.firebase_database_url, id_token
    );

    // Creamos el usuario que se va a crear en la DB
    let new_comment: Comment = Comment {
        author_uid: Some(user_claims.user_id),
        name: comment.name,
        timestamp: Utc::now().format("%d/%m/%Y %H:%M").to_string(),
        content: comment.content,
        url_img: comment.url_img,
        stars: comment.stars,
        like: 0,
        reply: Vec::new(),
        users_liked: Vec::new(),
    };

    // Enviamos el comentario a la base de datos para su creación
    match state
        .firebase_options
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

// Editar un comentario
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims, comment),
    fields(
        comment_id = %comment_id,
        user_id = %user_claims.user_id,
        comment_content_length = %comment.content.len(),
        operation = "edit_comment"
    )
)]
pub async fn edit_comment(
    Path(comment_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    Json(comment): Json<UpdateComment>,
) -> impl IntoResponse {
    // Obtenemos el ID del comentario a editar
    let url_firebase_db: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    // Verificar si el comentario existe
    let existing_comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Verificamos que el usuario sea el autor del comentario
    if existing_comment.author_uid != Some(user_claims.user_id) {
        return (
            StatusCode::FORBIDDEN,
            Json(ResponseAPI::<()>::error(
                "You are not authorized to edit this comment".to_string(),
            )),
        )
            .into_response();
    }

    // Creamos el nuevo comentario con los datos actualizados, solo cambiamos el timestamp contenido y stars
    let updated_comment: Comment = Comment {
        timestamp: Utc::now().format("%d/%m/%Y %H:%M").to_string(),
        content: comment.content,
        stars: comment.stars,
        ..existing_comment
    };

    // Intentamos actualizar el comentario en la base de datos
    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&updated_comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(comment) => (
                StatusCode::OK,
                Json(ResponseAPI::<Comment>::success(
                    "Comment updated successfully".to_string(),
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
                "Failed to update comment".to_string(),
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
    let url_firebase_db: String = format!(
        "{}/comments.json",
        state.firebase_options.firebase_database_url
    );

    // Realizamos la petición a la base de datos
    match state
        .firebase_options
        .firebase_client
        .get(&url_firebase_db)
        .send()
        .await
    {
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
    Extension(user_claims): Extension<UserAuthentication>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtenemos el ID del comentario a eliminar
    let url_firebase_db: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    // Verificar si el comentario existe
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

    println!("Comment Author UID: {:?}", comment.author_uid);
    if comment.author_uid != Some(user_claims.user_id) {
        return (
            StatusCode::FORBIDDEN,
            Json(ResponseAPI::<()>::error(
                "You are not authorized to delete this comment".to_string(),
            )),
        )
            .into_response();
    }

    // Intentamos eliminar el comentario
    match state
        .firebase_options
        .firebase_client
        .delete(&url_firebase_db)
        .send()
        .await
    {
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
        state.firebase_options.firebase_database_url, comment_id, id_token
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
        .firebase_options
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
    Json(reply_comment): Json<ReplyComment>,
) -> impl IntoResponse {
    // Obtenemos el comentario padre
    let mut comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // URL del comentario en Firebase
    let url_firebase_db_comment = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    // Generar ID único para la nueva reply
    let reply_id = Uuid::new_v4().to_string();

    // Crear la nueva reply con todos los campos
    let new_reply = ReplyComment {
        id: reply_id,
        author_uid: user_claims.user_id.clone(),
        name: reply_comment.name.clone(),
        timestamp: Utc::now().format("%d/%m/%Y %H:%M").to_string(),
        content: reply_comment.content.clone(),
        url_img: reply_comment.url_img.clone(),
        like: 0,
        users_liked: Vec::new(),
    };

    // Agregar la reply al comentario
    comment.reply.push(new_reply.clone()); // ✅ clonamos para poder devolver luego

    // Guardar el comentario actualizado en Firebase
    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db_comment)
        .json(&comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(_) => (
                StatusCode::CREATED,
                Json(ResponseAPI::<ReplyComment>::success(
                    "Reply added successfully".to_string(),
                    new_reply, // ✅ devolvemos la reply creada
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error("Failed to add reply".to_string())),
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
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    // Realizamos la petición a Firebase Realtime Database
    match state
        .firebase_options
        .firebase_client
        .get(url_firebase_db)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(comment) => Some(comment),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

// Obtener un comentario por id
#[debug_handler]
#[instrument(
    skip(state),
    fields(
        comment_id = %comment_id,
        operation = "get_comment_by_id"
    )
)]
pub async fn get_comment_by_id(
    Path(comment_id): Path<String>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => (
            StatusCode::OK,
            Json(ResponseAPI::<Comment>::success(
                "Comment fetched successfully".to_string(),
                comment,
            )),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ResponseAPI::<()>::error("Comment not found".to_string())),
        )
            .into_response(),
    }
}

// Editar una respuesta específica
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims, reply_update),
    fields(
        comment_id = %comment_id,
        reply_id = %reply_id,
        user_id = %user_claims.user_id,
        operation = "edit_reply"
    )
)]
pub async fn edit_reply(
    Path((comment_id, reply_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
    Json(reply_update): Json<ReplyComment>,
) -> impl IntoResponse {
    // Obtenemos el comentario padre
    let mut comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Limitar scope del mutable borrow
    let updated_reply = {
        let reply = comment.reply.iter_mut().find(|r| r.id == reply_id);

        let reply = match reply {
            Some(r) => r,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(ResponseAPI::<()>::error("Reply not found".to_string())),
                )
                    .into_response();
            }
        };

        // Verificamos ownership
        if reply.author_uid != user_claims.user_id {
            return (
                StatusCode::FORBIDDEN,
                Json(ResponseAPI::<()>::error(
                    "You are not authorized to edit this reply".to_string(),
                )),
            )
                .into_response();
        }

        // Actualizamos contenido y timestamp
        reply.content = reply_update.content.clone();
        reply.timestamp = Utc::now().format("%d/%m/%Y %H:%M").to_string();

        reply.clone()
    }; // <- aquí termina el mutable borrow

    // Guardamos el comentario actualizado en Firebase
    let url_firebase_db = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&comment)
        .send()
        .await
    {
        Ok(response) => match handle_firebase_response::<Comment>(response).await {
            Ok(_) => (
                StatusCode::OK,
                Json(ResponseAPI::<ReplyComment>::success(
                    "Reply updated successfully".to_string(),
                    updated_reply, // devolvemos la reply editada
                )),
            )
                .into_response(),
            Err((status, error)) => (status, Json(ResponseAPI::<()>::error(error))).into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseAPI::<()>::error(
                "Failed to update reply".to_string(),
            )),
        )
            .into_response(),
    }
}

// Eliminar una respuesta reply específica
#[debug_handler]
#[instrument(
    skip(state, id_token, user_claims),
    fields(
        comment_id = %comment_id,
        reply_id = %reply_id,
        user_id = %user_claims.user_id,
        operation = "delete_reply"
    )
)]
pub async fn delete_reply(
    Path((comment_id, reply_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(id_token): Extension<String>,
    Extension(user_claims): Extension<UserAuthentication>,
) -> impl IntoResponse {
    tracing::debug!(
        "delete_reply called: comment_id={}, reply_id={}, user_id={}",
        comment_id,
        reply_id,
        user_claims.user_id
    );

    // 1️Obtener el comentario padre
    let mut comment: Comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => {
            tracing::debug!("Found parent comment with {} replies", comment.reply.len());
            comment
        }
        None => {
            tracing::debug!("Parent comment not found: {}", comment_id);
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Buscar el reply a eliminar
    let reply_index: Option<usize> = comment.reply.iter().position(|r| r.id == reply_id);
    tracing::debug!("Computed reply_index: {:?}", reply_index);

    if reply_index.is_none() {
        tracing::debug!(
            "Reply not found in comment: {} reply_id: {}",
            comment_id,
            reply_id
        );
        return (
            StatusCode::NOT_FOUND,
            Json(ResponseAPI::<()>::error("Reply not found".to_string())),
        )
            .into_response();
    }

    let index: usize = reply_index.unwrap();
    let reply: &ReplyComment = &comment.reply[index];
    tracing::debug!(
        "Reply at index {} has author_uid={}",
        index,
        reply.author_uid
    );

    // Comprobar permisos
    if reply.author_uid != user_claims.user_id.clone() {
        tracing::debug!(
            "Unauthorized delete attempt by user={} for reply author={}",
            user_claims.user_id,
            reply.author_uid
        );
        return (
            StatusCode::FORBIDDEN,
            Json(ResponseAPI::<()>::error(
                "You are not authorized to delete this reply".to_string(),
            )),
        )
            .into_response();
    }

    // Eliminar el reply del vector
    tracing::debug!(
        "Removing reply at index {} from comment {}",
        index,
        comment_id
    );
    comment.reply.remove(index);

    // Guardar los cambios en Firebase
    let url_firebase_db: String = format!(
        "{}/comments/{}.json?auth={}",
        state.firebase_options.firebase_database_url, comment_id, id_token
    );

    tracing::debug!(
        "Putting updated comment to Firebase URL: {}",
        url_firebase_db
    );

    match state
        .firebase_options
        .firebase_client
        .put(&url_firebase_db)
        .json(&comment)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            let body_text = response.text().await.unwrap_or_default();
            tracing::debug!(
                "Firebase PUT response status: {}, body: {}",
                status,
                body_text
            );

            if status.is_success() {
                (
                    StatusCode::NO_CONTENT,
                    Json(ResponseAPI::<()>::success_no_data()),
                )
                    .into_response()
            } else {
                tracing::error!(
                    "Failed to delete reply: firebase returned non-success status {}",
                    status
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseAPI::<()>::error(
                        "Failed to delete reply".to_string(),
                    )),
                )
                    .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to send PUT to Firebase: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseAPI::<()>::error(
                    "Failed to delete reply".to_string(),
                )),
            )
                .into_response()
        }
    }
}

// Obtener una respuesta específica por id
#[debug_handler]
#[instrument(
    skip(state, id_token),
    fields(
        comment_id = %comment_id,
        reply_id = %reply_id,
        operation = "get_reply_by_id"
    )
)]
pub async fn get_reply_by_id(
    Path((comment_id, reply_id)): Path<(String, String)>,
    Extension(id_token): Extension<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Obtener el comentario
    let comment = match get_comment_data(&comment_id, &id_token, &state).await {
        Some(comment) => comment,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ResponseAPI::<()>::error("Comment not found".to_string())),
            )
                .into_response();
        }
    };

    // Buscar la reply por ID
    if let Some(reply) = comment.reply.into_iter().find(|r| r.id == reply_id) {
        return (
            StatusCode::OK,
            Json(ResponseAPI::<ReplyComment>::success(
                "Reply fetched successfully".to_string(),
                reply,
            )),
        )
            .into_response();
    }

    // Si no se encuentra la reply
    (
        StatusCode::NOT_FOUND,
        Json(ResponseAPI::<()>::error("Reply not found".to_string())),
    )
        .into_response()
}
