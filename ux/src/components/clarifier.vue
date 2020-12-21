<template>
  <div class="clarify">
    <div :class="(applying ? 'resolving ' : '') + 'problem'">
      <p>line <span class="lineno">{{ problem.line }}</span>:
              <span class="line">{{ problem.target }}</span>:
              <span class="problem">{{ problem.error }}</span></p>
      <div class="resolution" v-if="replacements.length > 0">
        <ul class="changes">
          <li><del>{{ problem.value }}{{ problem.vars }}</del></li>
          <li v-for="(r,i) in replacements" :key="i"><ins>{{ r[0] }}x {{ r[1] }}{{ problem.vars }}</ins></li>
        </ul>
        <div>
          <button @click="apply()" rel="apply">✔ Apply</button>
        </div>
      </div>
      <div class="grid results">
        <span v-for="card in problem.cards" :key="card.id">
          <vcb-card :key="card.id" :card="card"></vcb-card>
          <span class="clarif" :data-n="counts[card.id]">
            <span class="band">
              <a href="#" rel="dec" @click="decrement(card)">◀</a>
              <span>{{ counts[card.id] }}</span>
              <a href="#" rel="inc" @click="increment(card)">▶</a>
            </span>
          </span>
        </span>
      </div>
    </div>
  </div>
</template>

<script>
import VcbCard from '@/components/card'

export default {
  name: 'vcb-clarifier',
  components: {VcbCard},
  props: ['problem'],
  data: function () {
    return {
      applying: false,
      replacements: [], // [[n, card], [n, card] ...]
      counts: {}
    };
  },
  created() {
    this.problem.cards.forEach(card => {
      this.counts[card.id] = 0;
    });
  },
  methods: {
    replace(n, card) {
      let o = {}; o[card.id] = n;
      Object.assign(this.counts, o);
      let spec = card.others
               ? card.set.code + ' *' + card.number + ' ' + card.name
               : card.set.code +                      ' ' + card.name;
      for (let i = 0; i < this.replacements.length; i++) {
        if (this.replacements[i][1] == spec) {
          if (n == 0) {
            this.replacements.splice(i,1);
          } else {
            this.replacements[i][0] = n;
          }
          this.replacements = [...this.replacements];
          return;
        }
      }
      if (n > 0) {
        this.replacements.push([n, spec]);
      }
    },

    increment(card) {
      event.preventDefault();
      this.replace(this.counts[card.id] + 1, card);
    },

    decrement(card) {
      event.preventDefault();
      this.replace(Math.max(0, this.counts[card.id] - 1), card);
    },

    apply() {
      this.applying = true;
      event.preventDefault();
      this.$emit('clarified', {
        problem: this.problem,
        replacements: this.replacements
      });
    }
  }
}
</script>

<style lang="scss" scoped>
.clarify {
  .results {
    width: 100%;
  }

  .clarif {
    position: absolute;
    width: 100%;
    height: 100%;
    padding-top: 27%;
    z-index: 5;
    text-align: center;
    font-size: 50px;
    box-sizing: border-box;
    transform: rotate(5deg);
    left: 0; top: 0;

    visibility: hidden;

    a {
      color: inherit;
      text-decoration: none;
      border-bottom: none;
      font-size: 80%;
    }

    .band {
      display: inline-block;
      width: 100%;
      line-height: 1;
      background-color: rgba(255,255,255, 0.7);
      padding: 0.5em 0;
    }
  }

  div.problem {
    &.resolving {
      opacity: 0.3;
    }
    .lineno {
      font-weight: bold;
      border-bottom: 2px solid #000;
    }

    .line {
      color: #fff;
      font-family: monospace;
      font-weight: bold;
      background-color: firebrick;
      padding: 2px 8px;
    }

    .problem {
      color: firebrick;
      font-style: italic;
    }

    .results > span {
      position: relative;
    }
  }

  .changes {
    width: 50%;

    ins, del {
      display: block;
      padding: 2px 8px;
      font-family: monospace;
      font-weight: bold;
      white-space: pre;
      overflow: hidden;
      text-decoration: none;
    }
    ins {
      background-color: lightgreen;
      color: darkgreen;
    }
    del {
      background-color: salmon;
      color: firebrick;
      text-decoration: line-through;
    }
  }
  .resolution {
    display: flex;
    gap: 1em;
    justify-content: flex-start;
    align-items: center;

    button[rel=apply] {
      color: #fff;
      background: linear-gradient(#5ae81a 0,#11a436 100%);
      border-color: #42bb44;
    }
  }

}

.results > span:hover .clarif,
.results .clarif:not([data-n="0"]) {
  visibility: visible;
}
</style>
