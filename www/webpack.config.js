const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/bootstrap.tsx",
  devtool: "inline-source-map",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.wasm$/, // only load WASM files (ending in .wasm)
        // only files in our src/ folder
        include: path.resolve(__dirname, "src"),
        use: [
          {
            // load and use the wasm-loader dictionary
            loader: require.resolve("wasm-loader"),
            options: {},
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js", ".tsx"],
  },
  devServer: {
      publicPath: "/",
      contentBase: "./public",
      hot: true
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
};
