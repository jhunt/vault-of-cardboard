<template>
  <form class="user" @submit.prevent="authenticate(username, password)">
    <h2>Sign In</h2>
    <p class="divert">New to Vault of Cardboard?  <a href="/signup" @click.prevent="$emit('redirect-signup')">sign up today</a>!</p>

    <div class="oops" v-if="error == 'authentication-failed'">
      Invalid username or password.
    </div>
    <div class="oops" v-else-if="error">
      Uh-oh, something broke pretty bad.  Tap <vcb-mana mana="2UU"></vcb-mana> and summon James, Fixer of Codes!
    </div>
    <div class="control">
      <label>Username</label>
      <input type="text" name="username" ref="username" autocomplete="off" v-model="username">
    </div>
    <div class="control">
      <label>Password</label>
      <input type="password" name="password" v-model="password">
    </div>
    <button type="submit" class="default safe">Sign in</button>
  </form>
</template>

<script>
import cardboard from '@/lib/cardboard/index'

import VcbMana from '@/components/mana'

export default {
  name: 'vcb-login',
  components: {VcbMana},
  data() {
    return {
      error: undefined,
      username: '',
      password: ''
    }
  },
  mounted() {
    this.$refs.username.focus()
  },
  methods: {
    authenticate(username, password) {
      cardboard.API.authenticate(username, password).then(data => {
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
