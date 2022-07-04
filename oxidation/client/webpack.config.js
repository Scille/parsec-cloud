const { VueLoaderPlugin } = require('vue-loader');

module.exports = {
  entry: {
    main: './src/main.ts'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        exclude: /node_modules/
      },
      {
        test: /\.vue$/,
        exclude: /node_modules/,
        use: [
          'vue-loader',
          'ts-loader'
        ]
      },
      {
        test: /\.css$/,
        use: [
          'vue-style-loader',
          'css-loader'
        ]
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin()
  ],
  resolve: {
    extensions: ['*', '.js', '.ts', '.vue', '.json']
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
};
