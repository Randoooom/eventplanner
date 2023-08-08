<!--
  -     Copyright (C) 2023 Fritz Ochsmann
  -
  -     This program is free software: you can redistribute it and/or modify
  -     it under the terms of the GNU Affero General Public License as published
  -     by the Free Software Foundation, either version 3 of the License, or
  -     (at your option) any later version.
  -
  -     This program is distributed in the hope that it will be useful,
  -     but WITHOUT ANY WARRANTY; without even the implied warranty of
  -     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  -     GNU Affero General Public License for more details.
  -
  -     You should have received a copy of the GNU Affero General Public License
  -     along with this program.  If not, see <https://www.gnu.org/licenses/>.
  -->

<template>
  <Datepicker
    inline
    dark
    :format-locale="de"
    preview-format="dd.MM.yyyy"
    :enable-time-picker="false"
    :start-time="defaultTime"
    utc="preserve"
    v-bind="$attrs"
  >
    <template #action-row="{ selectDate }">
      <v-btn
        v-if="!disableInput"
        variant="tonal"
        color="accent"
        style="width: 100%"
        @click="selectDate"
      >
        {{ $t("form.select") }}
      </v-btn>
    </template>

    <template v-if="customDay" #day="{ day, date }">
      <slot name="day" :day="day" :date="date" />
    </template>
  </Datepicker>
</template>

<script setup lang="ts">
import { ref } from "#imports";
import Datepicker from "@vuepic/vue-datepicker";
import "@vuepic/vue-datepicker/dist/main.css";
import de from "date-fns/locale/de/index.js";

defineProps({
  customDay: { type: Boolean, default: false },
  disableInput: { type: Boolean, default: false },
});
const defaultTime = ref({ hours: 0, minutes: 0, seconds: 0 });
</script>
