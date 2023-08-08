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
  <v-container v-if="visitor">
    <v-card class="pa-3" v-if="!voted">
      <v-card-title class="text-h3">
        {{ $t("page.vote.title", { username: visitor.username }) }}
      </v-card-title>

      <v-divider class="mt-2" />

      <v-card-text>
        <v-row>
          <v-col cols="12" class="text-h6">
            {{ $t("page.vote.subtitle", { event: visitor.event.title }) }}
          </v-col>

          <v-col cols="12">
            {{ $t("page.vote.choose") }}
          </v-col>

          <v-col cols="12">
            <FormDatePicker
              class="mt-2"
              v-model="dates"
              multi-dates
              :allowed-dates="visitor.event!.dates"
              :highlight="visitor.event!.dates"
            />
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions>
        <v-btn variant="text" color="accent" @click="vote">
          {{ $t("form.action.confirm") }}
        </v-btn>
      </v-card-actions>
    </v-card>

    <v-banner v-else color="success" icon="mdi-check">
      {{ $t("page.vote.response.success") }}
    </v-banner>
  </v-container>
</template>

<script setup lang="ts">
import {
  onMounted,
  useLocalePath,
  useRoute,
  useRouter,
  useNuxtApp,
  ref,
} from "#imports";
import { Visitor } from "~/composables/types";
import { notificationEmitter } from "~/stores/notifications";
import moment from "moment";

const { $surrealdb } = useNuxtApp();

const notifications = notificationEmitter();
const route = useRoute();
const localePath = useLocalePath();
const id = route.query.visitor as string | undefined;

const voted = ref(false);
const visitor = ref(undefined as Visitor | undefined);
const dates = ref([]);

onMounted(async () => {
  if (!id) {
    return await useRouter().push(localePath("/"));
  }

  // login the visitor
  await $surrealdb.visit(id);
  // fetch the visitor
  visitor.value = await $surrealdb.connection
    .query('SELECT * FROM type::thing("visitor", $id) FETCH event', {
      id: id,
    })
    .then((value: any) => value.map(Object.fromEntries)[0]);
  visitor.value.event = Object.fromEntries(visitor.value.event);
  // set the currently selected dates
  // @ts-ignore
  dates.value = visitor.value!.visitable;
});

async function vote() {
  try {
    await $surrealdb.connection.merge(visitor.value!.id, {
      visitable: dates.value.map((date: string) =>
        moment.utc(date).toISOString()
      ),
    });

    notifications.attachNotification({
      icon: "mdi-check",
      color: "success",
      title: "page.vote.response.success",
    });
    voted.value = true;
  } catch {
    notifications.attachNotification({
      icon: "mdi-alert",
      color: "error",
      title: "page.vote.response.error",
    });
  }
}
</script>
