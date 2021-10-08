import Vue from "vue/dist/vue.js";
import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    wasm.load();
    let state = wasm.get_state();
    let input = wasm.get_input();
    let presets = wasm.get_preset_saves();
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        input: input,
        world: world,
        presets: presets,
        paused: false,
      },

      mounted: function () {
        let self = this;
        setInterval(function () {
          if (self.paused) {
            return;
          }

          wasm.tick();
          self.state = wasm.get_state();
          self.input = wasm.get_input();
        }, 100);
      },
      methods: {
        save: function () {
          wasm.save();
        },
        hard_reset: function () {
          wasm.hard_reset();
        },
        load: function () {
          wasm.load();
        },
        tick: function (work_name) {
          wasm.tick();
        },
        set_work: function (work_name) {
          wasm.set_work(work_name);
        },
        set_housing: function (housing_name) {
          wasm.set_housing(housing_name);
        },
        buy_tier: function (index) {
          wasm.buy_tier(index);
        },
        load_preset: function (preset) {
          wasm.set_preset_saves(preset);
        },
        rebirth: function () {
          wasm.do_rebirth();
        },
        toggle_pause: function () {
          this.paused = !this.paused;
        },
        can_buy_tier: function (tier) {
          return wasm.can_buy_tier(tier.level);
        },
        getAge: function (total_days) {
          const years = Math.floor(total_days / 365);
          const days = total_days % 365;

          if (years === 0) {
            return `${days} days`;
          } else if (days === 0) {
            return `${years} years`;
          }
          return `${years} years and ${days} days`;
        },
        getLifespan: function (total_days) {
          const years = Math.floor(total_days / 365);
          const days = total_days % 365;

          if (years === 0) {
            return `${days} days`;
          } else if (days === 0) {
            return `${years} years`;
          }
          return `${years} years and ${days} days`;
        },
        prettyPrint: function (value) {
          if (typeof value !== "number") {
            return value;
          }

          if (value < 1000) {
            return Number.parseFloat(value.toFixed(1)).toString();
          }

          const ending = ["K", "M", "B", "T", "Qa", "Qi", "He", "Se", "Oc", "No", "De"];
          let index = -1;
          while (value >= 1000 && index < ending.length - 1) {
            value /= 1000;
            index++;
          }

          return Number.parseFloat(value.toFixed(1)).toString() + ending[index];
        },
      },
    });
  })
  .catch(console.error);
