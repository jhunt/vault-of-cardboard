<template>
  <div class="landing">
    <div><img src="/img/front.png"></div>
    <div>
      <p>Whether you're putting together
      <router-link :to="search('type:sliver')">that Sliver deck,</router-link>
      seeking out
      <router-link :to="search('type:legendary and type:creature')">your next Commander,</router-link>
      or
      <router-link :to="search('Didgeridoo or type:Minotaur')">just looking for some jank</router-link>,
      <em>Vault of Cardboard</em> has got you covered.</p>

      <template v-if="mode == 'landing'">
        <p>With a <strong>powerful and flexible query language</strong> that lets you refine
        queries until you find <em>exactly</em> what you're looking for, and a
        <strong>buy / sell / trade</strong> model of <strong>collection management</strong>,
        you'll never again wonder what you have or what you need.</p>

        <div class="buttons">
          <button rel="signup" class="action" @click.prevent="mode = 'signup'">Sign up</button>
          <button rel="signin" class="action" @click.prevent="mode = 'signin'">Sign in</button>
        </div>
      </template>
      <vcb-signup v-else-if="mode == 'signup'" @redirect-signin="mode = 'signin'"></vcb-signup>
      <vcb-signin v-else-if="mode == 'signin'" @redirect-signup="mode = 'signup'"></vcb-signin>

      <vcb-footer></vcb-footer>
    </div>
  </div>
</template>

<script>
import VcbFooter from '@/components/footer'
import VcbSignin from '@/components/signin'
import VcbSignup from '@/components/signup'

export default {
  name: 'vcb-home-view',
  components: {
    VcbFooter,
    VcbSignin,
    VcbSignup
  },
  data() {
    return {
      mode: 'landing'
    }
  },
  methods: {
    search(q) {
      return {
        name: 'search',
        params: {
          query: q
        }
      }
    }
  }
}
</script>

<style lang="scss" scoped>
.landing {
  margin: 5vh 10vw;
  padding-bottom: 3em;
  overflow: none;
  display: flex;
  flex-flow: row wrap;

  img {
    max-width: 700px;
    width: 100%;
  }

  > div {
    width: 50%;

    &:nth-child(2) {
      font-size: 14pt;
      line-height: 1.4em;
      padding: 8em 0 0 4em;
      box-sizing: border-box;
    }
  }

  div {
    p {
      margin-bottom: 1em;
    }

    a {
      text-decoration: none;
      color: #0071bc;
      border-bottom: 1px dotted #0071bc;
    }

    strong {
      font-weight: bold;
      font-family: sans-serif;
      font-size: 90%;
      padding: 2px 6px;
      background-image: url(/img/hili.png);
      background-repeat: no-repeat;
      background-size: cover;
    }

    em {
      font-style: italic;
      font-weight: bold;
    }

    footer {
      position: absolute;
      bottom: 0;
      right: 2em;
      font-family: sans-serif;
      font-size: 9pt;
      opacity: 0.3;
      line-height: 1.1em;
      text-align: right;
    }
  }

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
}
</style>
