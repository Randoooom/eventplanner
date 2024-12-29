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
  <FormDialog v-model="dialog">
    <template #activator="{ activate }">
      <slot name="activator" :activate="activate" />
    </template>

    <v-card-title>
      {{ event.title }}
    </v-card-title>

    <v-card-text>
      <LazyFormDatePicker
        custom-day
        :allowed-dates="event.dates"
        :highlight="event.dates"
        disable-input
        v-if="!isXs"
      >
        <template #day="{ day, date }">
          <v-tooltip
            v-if="isInList(date).value && votes.length > 0"
            open-delay="100"
            :text="text(date).value"
          >
            <template #activator="{ props }">
              <FormFloatingDialog v-model="dates[day]">
                <template #activator="{ activate }">
                  <span
                    v-bind="props"
                    :class="votesFor(date).value!.by.length > 0 ? 'text-green' : 'text-red'"
                    @click="activate"
                  >
                    {{ day }}
                  </span>
                </template>

                <v-card-title>
                  {{
                    $t("page.dashboard.event.votes", {
                      date: date.toLocaleDateString(),
                    })
                  }}
                </v-card-title>

                <v-card-text>
                  <v-table>
                    <thead>
                      <tr>
                        <th>
                          {{ $t("visitor.list.name") }}
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr
                        v-for="(visitor, i) in votesFor(date).value!.by"
                        :key="i"
                      >
                        <td class="text-left">
                          {{ visitor }}
                        </td>
                      </tr>
                    </tbody>
                  </v-table>
                </v-card-text>
              </FormFloatingDialog>
            </template>
          </v-tooltip>

          <template v-else>
            {{ day }}
          </template>
        </template>
      </LazyFormDatePicker>

      <v-table v-else>
        <thead>
          <tr>
            <th>{{ $t("page.vote.overview.date") }}</th>
            <th>{{ $t("page.vote.overview.votes") }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(voting, i) in votes" :key="i">
            <td>{{ voting.date.toLocaleDateString() }}</td>
            <td>
              {{ voting.by.length }}
              <span class="text-grey">
                {{ voting.by.join(", ") }}
              </span>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card-text>
  </FormDialog>
</template>

<script lang="ts" setup>
import { ref, useNuxtApp, watch, computed, useI18n } from "#imports";
import moment from "moment";

interface VoteResult {
  date: Date;
  by: string[];
}

const { t } = useI18n();
const { $surrealdb } = useNuxtApp();
const dates = ref(new Array(31).fill(false));

const props = defineProps({
  event: { type: Object, required: true },
});
const dialog = ref(false);
const votes = ref([] as VoteResult[]);

watch(dialog, async (value: boolean) => {
  if (value) {
    votes.value = [];
    await fetch();
  }
});

const isInList = (date: Date) =>
  computed(() =>
    props.event.dates.some(
      (r: string) =>
        moment.utc(r).toDate().toDateString() === date.toDateString()
    )
  );

const votesFor = (date: Date) =>
  computed(() =>
    votes.value.find((vote) => vote.date.toDateString() === date.toDateString())
  );

const text = (date: Date) =>
  computed(() => {
    const voting = votesFor(date).value!;

    return t("page.dashboard.event.tooltip", voting.by.length);
  });

async function fetch() {
  votes.value = await $surrealdb.connection
    .query("SELECT * FROM fn::vote_overview(type::thing('event', $event))", {
      event: props.event.id.id,
    })
    .then((value: any) =>
      value[0].map((vote: VoteResult) => {
        vote.date = new Date(vote.date);

        return vote;
      })
    );
}
</script>
