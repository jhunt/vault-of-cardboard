<template>
  <span :class="cssClass" @click.prevent="$emit('click', $event)">
    <span v-if="backed" class="back"><span><img src="/img/mtgback.jpg"></span></span>
    <span :class="'face '/* + card.layout*/">
      <span><a v-if="dualFaced" href="#" rel="flip" @click.prevent="flipped = !flipped"></a>
      <img :src="img" :alt="altText" :title="altText" loading="lazy"></span>
    </span>
  </span>
</template>

<script>
export default {
  name: 'vcb-card',
  props: ['card', 'backed', 'sleeve'],
  data() {
    return {
      flipped: false,
    };
  },
  computed: {
    cssClass() {
      return "card " + (this.sleeve ? "sleeved sl-"+this.sleeve : '')
    },
    dualFaced() {
      return this.card.layout == 'transform'
          || this.card.layout == 'modal_dfc'
          || this.card.layout == 'double_faced_token'
    },
    img() {
      return '/cards/' + (this.flipped ? this.card.back : this.card.image)
    },
    altText() {
      return this.card.name + ' [' + (this.card.set ? this.card.set.code : '???') + ']'
    }
  }
}
</script>

<style lang="scss" scoped>
.card {
  display: block;
  position: relative;
  cursor: pointer;
  transform: rotate(5deg);
  transform-origin: 50% 50%;
  clip-path: url(#card-clip);

  @media only screen and (max-width: 759px) {
    transform: none;
  }

  .face, .back {
    display: block;

    span {
      display: block;

      overflow: hidden;

      background-image: url(/img/mtgback.jpg);
      background-size: cover;

      img {
        display: block;
        width: 100%;
        overflow: hidden;

        &.flipped {
          transform: rotate(180deg);
        }
      }

      a[rel=flip] {
        display: block;

        position: absolute;
        z-index: 6;

        background-image: url(/img/flip.png);
        background-size: cover;
        background-color: #ffffffb3;
        border-radius: 100%;

        width: 17%;
        height: 12%;
        right: 3%;
        bottom: 8%;
      }
    }
  }

  .face {
    background-image: url(/img/mtgblank.jpg);
    background-size: cover;

    position: relative;
    z-index: 1;
  }

  .back {
    position: absolute;
    transform: rotate(-5deg);
  }

  &.sleeved {
    .face {
      background: #6f6565;
      border: 1px solid #1c1b1b;
      border-radius: 0;
      margin: 2vh 0.3vw;
      padding: 0.25vw;
    }

    &.sl-n .face { background-color: #1110;   border-color: #1110; }
    &.sl-w .face { background-color: #ece7cb; border-color: #979068; }
    &.sl-u .face { background-color: #0b80bc; border-color: #243662; }
    &.sl-b .face { background-color: #444444; border-color: #111111; }
    &.sl-r .face { background-color: #b22222; border-color: #771515; }
    &.sl-g .face { background-color: #096e47; border-color: #122f05; }

    &.sl-ur .face {
      border-color: #111;
      background: linear-gradient(-60deg, #b22222, #b22222 40%, #0b80bc 60%, #0b80bc 100%);
    }
  }

}
</style>
