FROM node:latest AS build
WORKDIR /src
COPY ux/ .
RUN yarn install
RUN yarn build

FROM nginx:1
COPY --from=build /src/dist /vault-of-cardboard
COPY oci/ux/nginx.conf /etc/nginx/nginx.conf
# vim:ft=Dockerfile
