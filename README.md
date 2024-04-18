# ChiisanaURL - 小さなURL

Um encurtador de URL leve, simples, customizável e feito em RUST com suporte a senhas e URLs customizadas.

Todo cache das URLs é feito em Redis visando um melhor desempenho.

ChiisanaURL é customizável por meio de um arquivo properties que segue o padrão Java. Ex:

```
chars=ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~?#@!$&'*+,;=
path_length=4
base_url=localhost/
port=8080
user_redis=
password_redis=
endpoint_redis=127.0.0.1
port_redis=6379
password_size=8
```

* chars -> Conjunto de chars usado para geração de senhas e URLs;
* base_url -> Parte inicial da URL imutável, normalmente seu host;
* path_length -> Tamanho do path que vem após base_url;
* password_size -> tamanho do password gerado com base em chars;

Para inicializar o encurtador com o seu arquivo properties :

```
chiisanaURL.exe -properties application.properties
```

Caso não seja indicado um arquivo, o programa irá procurar o arquivo no mesmo local do executável.

O sistema tem 6 endpoints:

* POST com payload "Request" em / que cria a URL encurtada com base em url;
* POST com payload "Request" em /custom quer cria a URL encurtada customizada caso não exista ainda a URL;
* POST com payload "Request" em /password que cria a URL encurtada com base em url e devolve a senha;
* POST com payload "Request" em /post_password_custom_url que cria a URL encurtada com base em custom_path e devolve a
  senha;
* POST com payload "PasswordRequest" em /code devolve a URL protegida por senha;
* GET que recebe uma String representando o path encurtado em / e devolve url.
  Caso a URL esteja protegida por senha a requisição irá retornar "TOP SECRET".

Payloads:

```
PasswordRequest -> {
    "password" : "0@TQP'~X",
    "url" : "localhost/EAVB"
}

Request -> {
    "url" : "https://www.linkedin.com/in/alvarowm/",
    "custom_path" :"MEU_LINK"
}
```

Resposta:

```
Response -> {
    "url" : "localhost/12X34",
    "password" :"S3PYF4,E"
}
```







