<template>
  <div>
    <header>
      <h1>FIS falsch</h1>
    </header>
    <main>
      <report-form @submit="submitForm"/>
    </main>
    <footer>
      Uses data from <a href="https://opentransportdata.swiss">opentransportdata.swiss</a> for form input suggestions.
    </footer>
  </div>
</template>

<script>
  import ReportForm from './ReportForm';

  export default {
    name: 'app',
    components: {
      ReportForm
    },
    methods: {
      submitForm(data) {
        //TODO send data to backend.
      }
    }
  }
</script>

<!-- CSS libraries -->
<style src="normalize.css/normalize.css"></style>

<!-- Global CSS -->
<style>
  code {
    font-family: Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif;
    font-size: 0.9em;
    white-space: pre-wrap;
    color: #2c3e50;
  }

  code::before, code::after {
    content: '`';
  }
</style>

<!-- Scoped component css -->
<!-- It only affect current component -->
<style scoped>
</style>
