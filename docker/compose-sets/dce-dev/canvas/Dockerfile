FROM node:latest

WORKDIR /app/src/

COPY package*.json ./

RUN npm install

EXPOSE 8088

CMD ["npm", "run", "dev:canvas"]