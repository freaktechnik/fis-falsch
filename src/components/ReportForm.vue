<template>
  <form @submit="submit">
    <datalist id="stationNames">
      <option v-for="station in stations">{{ station }}</option>
    </datalist>
    <section>
      <label>Where</label>
      <label><input type="radio" value="train" @change="setType" :checked="isTrain" :disabled="disabled"> Train</label>
      <label><input type="radio" value="station" @change="setType" :checked="isStation" :disabled="disabled"> Station</label>
    </section>
    <fieldset v-if="isTrain">
      <station-selector label="From" :value="from" @input="setFrom" :required="isTrain" :disabled="disabled"/>
      <station-selector label="To" :value="to" @input="setTo" :required="isTrain" :disabled="disabled"/>
      <section>
        <label>Train</label>
        <select :disabled="!trains.length || disabled" :value="train" @change="setTrainNumber">
          <option v-for="train in trains">{{ train }}</option>
        </select>
      </section>
    </fieldset>
    <fieldset v-if="isStation">
      <station-selector label="Station" :value="station" @input="setStationName" :required="isStation" :disabled="disabled"/>
      <!-- Location? -->
    </fieldset>
    <section>
      <label>Defect Type</label>
      <select :value="defectType" @change="setDefectType" :disabled="disabled">
        <option value="incorrect">FIS Angaben inkorrekt</option>
        <option value="defect">FIS ausser Betrieb</option>
      </select>
    </section>
    <button type="submit">Submit</button>
  </form>
</template>

<script>
import StationSelector from './StationSelector';

export default {
  name: 'report-form',
  components: {
    StationSelector
  },
  props: {
    disabled: {
      type: Boolean,
      default() {
        return false;
      }
    }
  },
  data() {
    return {
      type: 'train',
      from: '',
      to: '',
      trains: [],
      train: 0,
      station: '',
      stations: [], //shared cache of stations?
      defectType: 'incorrect'
    };
  },
  methods: {
    resetData() {
      this.from = '';
      this.to = '';
      this.train = 0;
      this.station = '';
    },
    setType(e) {
      this.type = e.target.value;
      this.resetData();
    },
    setFrom(e) {
      //TODO get trains for that if to is set too
      this.from = e.target.value;
    },
    setTo(e) {
      //TODO get trains for that if from is set too
      this.to = e.target.value;
    },
    setTrainNumber(e) {
      this.train = e.target.value;
    },
    setStationName(e) {
      this.station = e.target.value;
    },
    setDefectType(e) {
      this.defectType = e.target.value;
    },
    submit() {
      this.$emit("submit", {
        type: this.type,
        train: this.train,
        station: this.station,
        defectType: this.defectType
      });
    }
  },
  computed: {
    isTrain() {
      return this.type == 'train';
    },
    isStation() {
      return this.type == 'station';
    }
  }
};
</script>

<style scoped>
</style>
