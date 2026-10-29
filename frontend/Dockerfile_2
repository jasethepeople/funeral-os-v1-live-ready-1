FROM node:20-alpine
WORKDIR /app
COPY package.json ./
RUN npm install --legacy-peer-deps
COPY . .
RUN npx tailwindcss -i ./app/globals.css -o ./app/output.css --minify || true
EXPOSE 3000
CMD ["npm", "run", "dev"]
