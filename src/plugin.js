module.exports = function (context, options) {
  return {
    configureWebpack(config, isServer, utils) {
      return {
        module: {
          rules: [
            { test: /\.rs$/, use: 'raw-loader' },
          ],
        },
      };
    },
  };
};
