
module.exports = {
  apps: [
    {
      name: "messenger",
      script: "bacon",
      args: "watch -w messenger -- cargo run --manifest-path messenger/Cargo.toml",
      watch: ["../messenger"],
      ignore_watch: ["node_modules", "logs"],
      env: {
        RUST_LOG: "info",
        MESSENGER_PORT: process.env.MESSENGER_PORT,
      },
    },
    {
      name: "minioc",
      script: "bacon",
      args: "watch -w minioc -- cargo run --manifest-path minioc/Cargo.toml",
      watch: ["../minioc"],
      ignore_watch: ["node_modules", "logs"],
      env: {
        RUST_LOG: "info",
        MINIOC_PORT: process.env.MINIOC_PORT,
      },
    },
    // Add more apps (like query, matcher, etc.) as needed
  ],
};
