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
  <v-container>
    <span class="text-h2 font-weight-bold">
      {{ $t("page.dashboard.title") }}
    </span>

    <v-row class="ma-12">
      <v-col
        cols="12"
        sm="12"
        md="6"
        lg="4"
        v-for="event in events"
        :key="event.id"
      >
        <v-card class="pa-2 overflow-visible" height="132px">
          <v-card-title class="overflow-visible">
            <v-row>
              <v-col cols="6">
                {{ event.title }}
              </v-col>

              <v-col
                cols="6"
                class="d-flex align-center justify-end overflow-visible"
              >
                <VoteOverview :event="event">
                  <template #activator="{ activate }">
                    <v-icon
                      icon="mdi-calendar"
                      size="small"
                      @click="activate"
                    />
                  </template>
                </VoteOverview>
              </v-col>
            </v-row>
          </v-card-title>

          <v-card-actions class="mt-5">
            <VisitorList :event="event.id">
              <template #activator="{ activate }">
                <v-btn variant="tonal" color="accent" @click="activate">
                  {{ $t("page.dashboard.event.visitors") }}
                </v-btn>
              </template>
            </VisitorList>

            <v-spacer />

            <EventEditor @refresh="fetch" :event="event" custom-activator>
              <template #activator="{ activate }">
                <v-btn variant="tonal" color="accent" @click="activate">
                  {{ $t("form.action.edit") }}
                </v-btn>
              </template>
            </EventEditor>
          </v-card-actions>
        </v-card>
      </v-col>

      <v-col cols="12" sm="12" md="6" lg="4">
        <v-card class="pa-2 overflow-visible" color="accent">
          <v-card-title
            class="d-flex justify-center align-center overflow-visible"
          >
            <EventEditor @refresh="fetch">
              <template #activator="{ activate }">
                <v-icon
                  @click="activate"
                  icon="mdi-plus-thick"
                  size="100px"
                  id="add-event"
                />
              </template>
            </EventEditor>
          </v-card-title>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { onMounted, ref, useNuxtApp, definePageMeta } from "#imports";
import { PlannedEvent } from "~/composables/types";

// only allow for 'account' scoped sessions
definePageMeta({
  middleware: ["auth"],
});

const { $surrealdb } = useNuxtApp();
const events = ref([] as PlannedEvent[]);

onMounted(async () => {
  await fetch();
});

async function fetch() {
  events.value = await $surrealdb.connection
    .select("event")
    .then((value: any) => value);
}
</script>

<style lang="sass" scoped>
#add-event
  opacity: 0.6

.v-card
  transition: transform 0.1s

  &:hover
    transform: scale(1.06)
</style>
