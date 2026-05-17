FROM node:22 AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
ENV CI=true
ENV NODE_ENV=production
RUN corepack enable

FROM base AS build
WORKDIR /app
COPY . .
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm nx run @streamforge/web:build

FROM node:22-slim AS runner
ENV NODE_ENV=production
WORKDIR /app
COPY --from=build --chown=node:node /app/apps/web/.output ./
USER node
EXPOSE 3000
CMD ["node", "server/index.mjs"]
