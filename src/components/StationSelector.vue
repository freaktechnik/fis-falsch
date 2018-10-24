<template>
  <section>
    <label :for="name">{{ label }}</label>
    <input type="text" :id="name" :name="name" :value="value" @input="onInput" :list="name+'-list'" :disabled="disabled" :required="required">
    <datalist :id="name+'-list'">
      <option v-for="station in stations">{{ station }}</option>
    </datalist>
  </section>
</template>

<script>
export default {
  name: 'station-selector',
  props: {
    label: String,
    name: String,
    value: String,
    disabled: {
      type: Boolean,
      default: false
    },
    required: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      value: '',
      stations: []
    };
  },
  methods: {
    onInput(e) {
      this.value = e.target.value;
      this.$emit('input', this.value);
      //TODO fetch 10 best matches from backend (resp. service worker cache)
    }
  }
};
</script>
