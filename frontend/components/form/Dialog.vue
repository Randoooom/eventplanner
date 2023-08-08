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
  <div class="floating-form">
    <v-tooltip v-if="tooltip" open-delay="500" :text="$t(tooltip)">
      <template #activator="{ props }">
        <v-icon
          :icon="icon"
          :color="color"
          @click="active = !active"
          v-bind="props"
          :size="size"
          :class="iconClass"
        />
      </template>
    </v-tooltip>

    <slot :activate="() => (active = !active)" name="activator" />

    <v-dialog v-model="active">
      <v-card
        v-click-outside="onClickOutside"
        class="ma-auto overflow-visible"
        max-width="100%"
      >
        <slot />
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed } from "#imports";

const props = defineProps({
  tooltip: { type: String, required: false },
  icon: { type: String, required: false },
  disabled: { type: Boolean, default: false },
  color: { type: String, default: "white" },
  size: { type: String, default: undefined },
  iconClass: { type: String, default: undefined },
  modelValue: { type: Boolean, required: true },
});
const emit = defineEmits(["update:model-value"]);
const active = computed({
  get() {
    return props.modelValue;
  },
  set(value: boolean) {
    emit("update:model-value", value);
  },
});

// eslint-disable-next-line require-jsdoc
function onClickOutside() {
  if (active.value) active.value = false;
}
</script>
