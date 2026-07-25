# Stage 1: build and prepare dependencies
FROM node:22-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm prune --omit=dev
# Stage 2: minimal runtime image
FROM node:22-alpine AS runner
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/src ./src
EXPOSE 3000
CMD ["node", "src/index.js"]