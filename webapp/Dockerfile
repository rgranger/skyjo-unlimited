FROM node:18.16-alpine
ENV NODE_ENV development
WORKDIR /app
COPY package.json package-lock.json ./
COPY . .
RUN npm i
