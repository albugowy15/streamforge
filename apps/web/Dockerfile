FROM node:22 AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
ENV CI=true
RUN corepack enable

FROM base AS build
WORKDIR /app
COPY . .
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm nx run @streamforge/web:build

FROM base AS runner
RUN addgroup --system --gid 1001 nodejs \
  && adduser  --system --uid 1001 appuser
WORKDIR /app
COPY --from=build /app/apps/web/.output .
USER appuser
EXPOSE 3000
CMD ["node", "server/index.mjs"]
