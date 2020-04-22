const colorize    = require('../helpers.js').colorize,
      API         = require('../api.js').API,
      When        = require('../when.js').When,
      CardsLoaded = require('../vault.js').CardsLoaded;

module.exports = {
  data() {
    return {
      error:    undefined,
      username: '',
      password: ''
    };
  },

  mounted() {
    $('[name=username]').focus();
  },

  methods: {
    colorize: colorize,

    authenticate(username, password) {
      event.preventDefault();

      API.authenticate(username, password).then(data => {
        if (data.authenticated) {
          $vault.unload_collection();
          When.trigger('authenticated');

          document.cookie = 'vcb_sesh='+data.authenticated.session+';max-age=7776000;samesite=strict';
          API.fetch_collection_for(data.authenticated.uid)
            .then(c => When(CardsLoaded, () => {
              $vault.load_collection(c.base, c.patches);
            }));

          console.log('emitting auth\'d event!');
          console.log('from ', this);
          this.$emit('authenticated', data.authenticated);

        } else {
          this.error = data.response.message;
        }
      });
    }
  },
  template: `
<div class="landing login">
  <div><img src="/img/front.png"></div>
  <div>
    <p>Whether you're putting together <a href="#!/q/type:sliver">that Sliver deck,</a>
    seeking out <a href="#!/q/type:legendary and type:creature">your next Commander,</a>
    or <a href="#!/q/Didgeridoo or type:Minotaur">just looking for some jank</a>,
    <em>Vault of Cardboard</em> has got you covered.</p>

    <form class="user" @submit="authenticate(username, password)">
      <h2>Sign In</h2>
      <p class="divert">New to Vault of Cardboard?  <a href="#!/signup">sign up today</a>!</p>

      <div class="oops" v-if="error == 'authentication-failed'">
        Invalid username or password.
      </div>
      <div class="oops" v-else-if="error">
        Uh-oh, something broke pretty bad.  Tap {{ colorize("2UUU") }} and summon James, Fixer of Codes!
        <!-- Uh-oh, something broke pretty bad.  Tap {{ colorize("4BR") }} to cast James Legendary Bug Fix -->
      </div>
      <div class="control">
        <label>Username</label>
        <input type="text" name="username" autocomplete="off" v-model="username">
      </div>
      <div class="control">
        <label>Password</label>
        <input type="password" name="password" v-model="password">
      </div>
      <button type="submit"class="default safe">Sign in</button>
    </form>
    <copyright-footer></copyright-footer>
  </div>
</div>
`
};
