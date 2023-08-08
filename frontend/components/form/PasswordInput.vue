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
  <v-text-field
    v-model="password"
    :append-inner-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
    :data-form-type="dataFormType"
    :label="$t(label)"
    :rules="[required($t)]"
    :type="show ? 'text' : 'password'"
    filled
    v-bind="$attrs"
    @click:append-inner="show = !show"
    :hint="strength ? score : undefined"
  />
</template>

<script lang="ts" setup>
import { ref, required, computed } from "#imports";
import zxcvbn from "zxcvbn";

const password = ref("");
const show = ref(false);

const { t } = useI18n();

defineProps({
  label: { type: String, required: false, default: "form.password.label" },
  dataFormType: { type: String, required: false, default: "password" },
  strength: { type: Boolean, required: false, default: false },
});

const score = computed<string>(() => {
  switch (zxcvbn(password.value).score) {
    case 0:
      return t("form.password.strength.0");
    case 1:
      return t("form.password.strength.1");
    case 2:
      return t("form.password.strength.2");
    case 3:
      return t("form.password.strength.3");
    case 4:
      return t("form.password.strength.4");
  }
});
</script>
