# cargo-prisma

The Unofficial Prisma CLI for Rust

## Install

```bash,no_run
cargo install --git https://github.com/chensoft/cargo-prisma
```

## Example

```bash,no_run
mkdir prisma

cat << EOF > prisma/schema.prisma
datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

generator client {
  provider = "cargo prisma"
  output   = "schema.rs"
}

model User {
  id String @id
}
EOF

cargo prisma generate

cargo prisma db push

cargo prisma migrate dev
```

## Future

Our version numbers are based on the prisma-client-rust's version numbers. We will continually track and align with their version changes if possible.