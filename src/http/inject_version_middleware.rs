use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware, WebContentType,
};

pub struct InjectVersionMiddleware;

#[async_trait::async_trait]
impl HttpServerMiddleware for InjectVersionMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        println!("path: {}", ctx.request.get_path().as_str());

        let Some(file_to_inject) = crate::app::APP_CTX.file_to_inject.as_ref() else {
            return None;
        };

        if ctx
            .request
            .get_path()
            .equals_to_case_insensitive(file_to_inject)
        {
            println!("Trying to Inject Version");
            let file_name = get_file_name(
                my_http_server::StaticFilesMiddleware::DEFAULT_FOLDER,
                file_to_inject,
            );

            let file = tokio::fs::read_to_string(&file_name).await;

            let file = match file {
                Ok(file) => file,
                Err(_) => return None,
            };
            println!("Replacing version to: {}", crate::app::APP_CTX.app_version);
            println!(
                "Replacing compile time to: {}",
                crate::app::APP_CTX.compile_time
            );
            let file_content = file
                .replace("${APP_VERSION}", &crate::app::APP_CTX.app_version)
                .replace("${APP_COMPILE_TIME}", &crate::app::APP_CTX.compile_time);

            let result = HttpOutput::from_builder()
                .set_content_type_opt(WebContentType::detect_by_extension(file_to_inject))
                .set_content(file_content.into_bytes())
                .into_ok_result(false);

            return Some(result);
        }

        None
    }
}

pub fn get_file_name(file_folder: &str, path: &str) -> String {
    format!("{}{}", file_folder, path)
}
