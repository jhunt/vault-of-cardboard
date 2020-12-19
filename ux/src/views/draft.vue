<template>
  <div class="draft prose">
    <h1>Draft!</h1>
    <div v-for="block in blocks" :key="block.id">
      <h2>{{ block.name }}</h2>
      <div class="banner" :style="`background-image: url(/draft/${block.id}/banner.jpg)`"></div>
      <div class="sets">
        <div v-for="draft in block.draft" :key="draft.set"
             @click.prevent='$router.push({ name: "draft-set", params: { set: draft.set }})'>
          <img :src="`/draft/${draft.set}/box.jpg`">
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      blocks: [
        {
          id: 'ISD',
          name: 'Innistrad',
          draft: [
            { set: 'ISD', name: 'Innistrad' },
            { set: 'DKA', name: 'Dark Ascension' },
            { set: 'AVR', name: 'Avacyn Restored' },
          ],
        },
        {
          id: 'XLN',
          name: 'Ixalan',
          draft: [
            { set: 'XLN', name: 'Ixalan' },
            { set: 'RIX', name: 'Rivals of Ixalan' },
          ],
        },
        {
          id: 'ICE',
          name: 'Ice Age Block',
          draft: [
            { set: 'ICE', name: 'Ice Age' },
            { set: 'ALL', name: 'Alliances' },
            { set: 'HML', name: 'Homelands' },
            { set: 'CSP', name: 'Coldsnap' }
          ]
        },
        {
          id: 'MIR',
          name: 'Mirage Block',
          draft: [
            { set: 'MIR', name: 'Mirage' },
            { set: 'VIS', name: 'Visions' },
            { set: 'WTH', name: 'Weatherlight' }
          ]
        }
      ]
    }
  }
}
</script>

<style lang="scss" scoped>
.banner {
  height: 200px;
  background-size: cover;
}
.sets {
  display: flex;
  flex-direction: row;
  justify-content: space-between;

  > div {
    padding: 1em;
  }

  img {
    max-height: 40vh;
  }
}
</style>
