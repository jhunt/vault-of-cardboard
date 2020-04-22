module.exports = Object.freeze(Object.assign({},
  require('./api.js'),
  require('./cdif.js'),
  require('./draft.js'),
  require('./helpers.js'),
  require('./query.js'),
  require('./vault.js'),
  require('./when.js'),
  {
    UI: {
      Docs:    require('./ui/docs.js'),
      Login:   require('./ui/login.js'),
      SetList: require('./ui/set-list.js')
    }
  }));
