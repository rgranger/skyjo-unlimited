FROM node:18.16-alpine as build
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm i
COPY . .
RUN npm run build

FROM node:18.16-alpine as deploy-node
WORKDIR /app
COPY --from=build /app/package*.json .
RUN npm ci --omit=dev
COPY --from=build /app/build .
CMD [ "node", "index.js" ]
