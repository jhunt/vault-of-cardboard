<template>
  <form class="user" @submit.prevent="signup()">
    <h2>Sign Up Today!</h2>
    <p class="divert">Already a proud Vaulter?  <a href="/signin" @click.prevent="$emit('redirect-signin')">sign in</a>!</p>

    <div class="oops" v-if="error == 'username-too-short'">
      The username you picked is too short.  Username really ought to be at least four (4) characters long.
    </div>
    <div class="oops" v-else-if="error == 'invalid-email'">
      Your email address doesn't look quite right; it should have an at sign ('@') and at least one dot ('.')...
    </div>
    <div class="oops" v-else-if="error == 'password-too-short'">
      The password you chose is too short; try for at least eight (8) characters, or use a password manager.
    </div>
    <div class="oops" v-else-if="error">
      Uh-oh, something broke pretty bad.<br>Pay <vcb-mana mana="2UU"></vcb-mana> and summon James, Fixer of Codes!
    </div>
    <div class="control">
      <label>Username</label>
      <input type="text" name="username" autocomplete="off" v-model="username"
             ref="username" placeholder="Pick a memorable username">
    </div>
    <div class="control">
      <label>Email</label>
      <input type="text" name="email" autocomplete="off" v-model="email"
             placeholder="We'll never spam you or sell you">
    </div>
    <div class="control">
      <label>Password</label>
      <input type="password" name="password" v-model="password"
             placeholder="Make it secret.  Make it secure.">
    </div>
    <button type="submit" class="default safe">Sign up!</button>
  </form>
</template>

<script>
import cardboard from '@/lib/cardboard/index'

import VcbMana from '@/components/mana'

export default {
  name: 'vcb-signup',
  components: {VcbMana},
  data() {
    return {
      error:    undefined,
      username: '',
      email:    '',
      password: ''
    }
  },
  mounted() {
    this.$refs.username.focus()
  },
  methods: {
    signup() {
      cardboard.API.signup(this.username, this.email, this.password)
        .then(data => {
          if (data.authenticated) {
            this.$store.dispatch('auth', data.authenticated)
            this.goto('/q/owned')

          } else {
            this.error = data.response.message

          }
        })
    }
  },
}
</script>

<style lang="scss" scoped>
form {
  margin: 2em auto;
  max-width: 44ex;

  div {
    margin: 1em 0;
    position: relative;

    a {
      position: absolute;
      top: 0px;
      right: 0px;
      font-size: 11pt;
      border: none;
      line-height: 1.1em;
      visibility: hidden;
    }
  }

  div:hover a {
    visibility: visible;
  }

  > div:last-child {
    display: flex;
    flex-flow: row-reverse;
  }
}
</style>
