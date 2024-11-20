use actix_web::{http::header, web, App, HttpResponse, HttpServer};

async fn saludo()-> HttpResponse{
   let html_content = r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>SERVIDOR WEB RUST</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    margin: 20px;
                }
                h1 {
                    color: #4CAF50;
                }
                form {
                    margin-top: 20px;
                }
                input, button {
                    margin: 5px 0;
                    padding: 10px;
                    font-size: 16px;
                }
                    button {
                    padding: 10px 20px;
                    font-size: 16px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 5px;
                    cursor: pointer;
                }
                button:hover {
                    background-color: #45a049;
                }
            </style>
        </head>
        <body>
            <h1>Formulario en Servidor de Rust</h1>
            <p>Ingresa tus datos para probar:</p>
            <form >
                <label for="nombre">Nombre:</label><br>
                <input type="text" id="nombre" name="nombre" placeholder="Tu nombre" required><br>
                <label for="email">Correo:</label><br>
                <input type="email" id="email" name="email" placeholder="Tu correo" required><br>
                <button onclick="mostrarAlerta()">Enviar</button>
            <script>
                function mostrarAlerta() {
                    alert('¡Bienvenido!');
                }
            </script>
            </form>
        </body>
        </html>
    "#;
   HttpResponse::Ok().content_type(header::ContentType::html()).body(html_content)
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
   HttpServer::new(|| App::new()
      .route("/", web::get().to(saludo)))
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}
