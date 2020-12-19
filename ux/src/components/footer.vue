<template>
  <footer>
    Magic: The Gathering is a Trademark of Wizards of the Coast, Inc. and Hasbro, Inc.<br/>
    Vault of Cardboard is Copyright &copy; {{ year }} <a href="https://jameshunt.us">James Hunt</a>
  </footer>
</template>

<script>
export default {
  name: 'vcb-footer',
  props: {
    year: {
      type: Number,
      default: () => 1900 + new Date().getYear()
    }
  }
}
</script>
