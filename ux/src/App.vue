<template>
  <div id="app" @click.stop.prevent="search_out = false">
    <header>
      <h1 @click="goto('/')">Vault of Cardboard</h1>
      <input name="q" placeholder="i.e.: Ral color:RU +draw" autocomplete="off"
            :value="query"
            @change="search($event.target.value)"
            @keyup="search($event.target.value, $event.keyCode != 13)">
      <nav>
        <li :class="search_out ? 'active' : ''"><a @click.prevent.stop="search_out = !search_out">Search</a>
          <nav>
            <li><router-link to="/docs">How do I search?</router-link></li>
            <li><router-link to="/sets">What sets are there?</router-link></li>
            <li class="separator"></li>
            <li v-if="session"><router-link :to='q("owned")'>My Collection</router-link></li>
            <li v-if="session"><router-link :to='q("own:2+")'>My Duplicates</router-link></li>
            <li v-if="session"><router-link :to='q("own:4+")'>My Playsets</router-link></li>
            <li v-if="session" class="separator"></li>
            <li><router-link :to='q("type:planeswalker and unique")'>Planeswalkers</router-link></li>
            <li><router-link :to='q("=mythic and !reprint")'>Mythics</router-link></li>
            <li><router-link :to='q("+gains? and +loses? and unique")'>Life Gain + Loss</router-link></li>
            <li><router-link :to='q("type:land and +you may pay 2 life and !reprint")'>Shock Lands</router-link></li>
            <li><router-link :to='q("usd:50+ and !reprint")'>Expensive Cards</router-link></li>
            <li><router-link :to='q("(legal:standard and unique)")'>Standard-Legal</router-link></li>
          </nav></li>
        <li v-if="!session"><router-link to="/signin">Sign in</router-link></li>
        <li v-if="session"><a class="-username">{{ session.username }}</a>
          <nav>
            <li><router-link to="/timeline">My Collection</router-link></li>
            <li v-if="session.cohort != 'stable'"><router-link to="/decks">Decks</router-link></li>
            <li v-if="session.cohort != 'stable'"><router-link to="/goals">Goals</router-link></li>
            <li><a href="/logout" @click.prevent="logout()">Sign out</a></li>
          </nav></li>
      </nav>
    </header>
    <div v-if="colorband && searching" class="color" :style="colorband"></div>
    <div id="main"><router-view></router-view></div>
    <svg width="1mm" height="1mm" viewBox="0 0 1 1" xmlns="http://www.w3.org/2000/svg">
      <clipPath clipPathUnits="objectBoundingBox" id="card-clip"><rect x="0" y="0" width="1" height="1" rx="0.040" ry="0.040" fill="none" stroke="#000000" stroke-width="0.5"></rect></clipPath>
    </svg>

    <!--
    <div class="modal-bg" v-if="modal"></div>
    <card-detail
      v-if="modal"
      :card="modalfg.card"></card-detail>
      -->
  </div>
</template>

<script>
import "mana-font"
import "keyrune"

import { mapGetters } from 'vuex'

import Log from '@/lib/log'
Log.console(Log.INFO)

export default {
  data() {
    return {
      search_out: false
    }
  },
  mounted() {
    Log.info('mounted; loading through $store...')
    this.$store.dispatch('load')
  },
  computed: {
    searching() {
      return this.$route.name == 'search'
    },
    ...mapGetters(['query', 'session', 'colorband'])
  },
  methods: {
    logout() {
      Log.info('logging out')
      this.$store.commit('session', null)
    },

    q(q) {
      return { name: 'search', params: { query: q } }
    },
    search(q, balk) {
      if (!balk) {
        Log.info(`initiating query [${q}]`)
        this.goto(this.q(q))
      }
    }
  }
}
</script>


<style lang="scss">
@import "./reset.scss";
@import "./legacy.scss";

body {
  min-height: 100vh;
  position: relative;
}

header {
  background-color: #000;
  color: #fff;
  font-family: Arial, sans-serif;
  font-weight: bold;

  display: flex;
  justify-content: space-between;

  box-shadow: 2px 3px 17px #000;

  h1 {
    padding: 16px;
    text-indent: -5000px;
    background-image: url(/img/logo.png);
    background-size: cover;
    width: 160px;
    height: 73px;
    padding: 0;
    margin: 8px 0 0 16px;
  }

  input {
    margin: 1em;
    color: #fff;
    background-color: #555;
    border: none;
    border-radius: 5px;
    font-size: 24px;
    padding: 8px;
    flex: auto;
    height: 1.2em;
    vertical-align: middle;
  }

  nav {
    a {
      text-decoration: none;
      color: inherit;
    }

    li nav li {
      &.separator {
        height: 1px;
        background-color: #fff;
        margin: 8px 0;
        opacity: 0.4;
      }
      &:hover {
        background-color: #000;
        color: yellow;
      }

      a {
        padding: 12px 24px;
        display: block;
      }
    }
  }

  > nav {
    list-style: none;
    display: flex;
    justify-content: flex-end;

    li {
      align-self: flex-end;
      &:hover {
        background-color: forestgreen;
      }

      nav {
        display: none;
        position: absolute;
        right: 0;
        background-color: #444;
        box-shadow: 5px 5px 5px #666;
      }
    }

    > li {
      a {
        display: block;
        padding: 16px;
      }

      &.active > nav,
      &:hover > nav {
        display: block;
        z-index: 500;
      }
    }
  }
}

/* forms.scss */
button {
  display: block;
  cursor: pointer;

  font-size: 11pt;
  font-weight: 600;

  line-height: 2em;
  margin: 1em 0;
  padding: 0.25em 1.5em;
  box-sizing: border-box;

  border-radius: 4px;
    border: 1px solid #ccc;

  &.action {
    border: 1px solid #ccc;
    color: #000;
    background: linear-gradient(#f8f8f8 0%, #ddd 100%);

    &:hover {
      color: yellow;
      background: linear-gradient(#69bef7 0,#0878ae 100%);
      border-color: #428bbb;
    }
  }
}

form,
.buttons {
  button {
    display: inline-block;
    margin: 4px 8px 4px 0;

    &[rel=signup] {
      color: yellow;
      background: linear-gradient(#69bef7 0,#0878ae 100%);
      border-color: #428bbb;
    }
  }
}

form.user,
form#login {
  font-family: sans-serif;

  h2 {
    font-size: 21pt;
    font-weight: bold;
    margin: 3em 0 0.6em 0;
  }

  p.divert {
    font-size: 9pt;
    font-style: italic;
  }

  .control {
    border: 1px solid #ccc;
    border-radius: 5px;
    overflow: hidden;
    padding: 5px;
    box-sizing: border-box;
    margin-top: 0;
    background-color: #1e90ff0a;

    > label {
      display: block;
      color: #333;
      font-size: 0.75rem;
      font-weight: bold;
      line-height: 1em;
    }

    input:not([type=radio]), textarea {
      width: 100%;
      padding: 8px 4px;
      box-sizing: border-box;

      color: inherit;
      background: transparent;

      font-weight: bold;
      font-size: 14pt;

      border-radius: 0;
      border-width: 0;
      outline: none;

      &::placeholder       { opacity: 0.2; }
      &:focus::placeholder { opacity: 0.5; }

      &::placeholder {
        font-weight: normal;
        font-style: italic;
      }
    }
  }

  button {
    color: #fff;
    background: linear-gradient(#69bef7 0,#0878ae 100%);
    border-color: #428bbb;

    &:hover {
      color: yellow;
    }
  }
}

form.data-entry {
  position: relative;
  padding-bottom: 5rem;

  .error {
    font-size: 0.85rem;
    line-height: 1.1em;
    color: firebrick;
    padding: 1em 0;
  }

  button {
    border: 1px solid #ccc;
    color: #000;
    background: linear-gradient(#f8f8f8 0%, #ddd 100%);

    &.safe {
      color: #fff;
      background: linear-gradient(#69bef7 0,#0878ae 100%);
      border-color: #428bbb;

      &:hover {
        color: yellow;
      }
    }
    &.danger {
      color: #aaa;
      opacity: 0.5;
      font-weight: normal;

      &:hover {
        opacity: 1.0;
        color: #b22222;
        border-color: #b22222;
      }
    }
  }

  .band {
    width: 100%;
    margin: 2em 0;
    display: flex;
    flex-direction: row;
    justify-content: space-between;

    > div:nth-child(1) { width: 60%; }
    > div:nth-child(2) { width: 40%; }

    .control {
      border: 1px solid #ccc;
      border-radius: 5px;
      overflow: hidden;
      padding: 5px;
      box-sizing: border-box;
      margin-top: 0;
      margin-left: 0;
      margin-right: 0;

      > label {
        display: block;
        color: #333;
        font-size: 0.75rem;
        font-weight: bold;
        line-height: 1em;
      }

      ul {
        list-style: none;
        margin: 0;

        li {
          font-size: 0.95rem;
          margin: 0.5em 0;
        }
      }

      input:not([type=radio]), textarea {
        width: 100%;
        padding: 8px 4px;
        box-sizing: border-box;

        color: inherit;
        background-color: inherit;

        font-weight: bold;
        font-size: 14pt;

        border-radius: 0;
        border-width: 0;
        outline: none;

        &::placeholder       { opacity: 0.2; }
        &:focus::placeholder { opacity: 0.5; }

        &::placeholder {
          font-weight: normal;
          font-style: italic;
        }
      }

      textarea {
        font-weight: normal;
        font-family: sans-serif;

        &.cdif {
          font-family: monospace;
          font-size: 0.95rem;
          min-height: 20em;
        }
      }

      &.main, &.import {
        width: 100%;
        background-color: #e7f7ea;
      }
    }

    .control {
      &.gained { background-color: #e7f7ea; }
      &.lost   { background-color: #f2f2f2; }
    }
    .gainloss {
      div.status {
        font-style: italic;
        font-size: 10pt;
        height: 1.75em;

        .in-progress { color: #0071bc; }
        .ok          { color: #088610; }
      }
    }

    .help {
      font-size: 0.85rem;
      line-height: 1.1em;
      padding: 0 1rem;
      box-sizing: border-box;

      &:not(.no-frills) {
        background-color: #1e90ff21;
        border-radius: 5px;
        margin-left: 2em;
        padding: 9px;
      }

      p { margin: 0; }
      p + p { margin-top: 1em; }
      p.example { padding-left: 1em; }

      .segment {
        padding: 4px;
        background-color: #eee;
        border: 2px dashed #ccc;
        border-radius: 4px;

        &.highlight {
          background-color: #c0eaec;
          border-color: #2fb9b3;
        }
      }
    }
  }

  .band.error {
    .control {
      border-left: 1em solid firebrick;
    }

    .error {
      font-size: 0.85rem;
      color: firebrick;

      padding: 0;
      border-top: 0.5px solid #ccc;
      margin: 0.25em 1em 0 0;
    }
  }
}
</style>
