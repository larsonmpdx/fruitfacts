* **the firewall should open ports `80` and `443`** (`80` is only for an http->https redirect)
* certbot will take our http-only nginx file and change it to https *and* add a block for http redirect
* the API server at port `8080` will also be accessed through port `443` and the URL will let nginx proxy to the right server (don't open port `8080` in the firewall)
