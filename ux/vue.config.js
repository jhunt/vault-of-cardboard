module.exports = {
  devServer: {
    port: 8099,
    disableHostCheck: true,
    proxy: 'http://127.0.0.1:8090'
  }
}
