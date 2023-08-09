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
    <v-row class="mt-16 pt-8">
      <v-col cols="12" md="10" lg="8">
        <p id="title" class="text-h2 font-weight-bold pb-8">
          {{ $t("page.home.title") }}
        </p>

        <p class="text-grey text-h6 font-weight-regular ml-6">
          {{ $t("page.home.subtitle") }}
        </p>
      </v-col>
    </v-row>

    <v-row class="mt-12">
      <v-col v-for="i in 3" :key="i" cols="12" sm="12" md="6" lg="4">
        <v-card height="110" class="pa-2 pb-6">
          <v-card-title>
            {{ $t(`page.home.cards.${i}.title`) }}
          </v-card-title>

          <v-card-text class="text-grey">
            {{ $t(`page.home.cards.${i}.subtitle`) }}
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" class="mt-6">
        <v-btn
          color="accent"
          class="text-none font-weight-bold"
          size="large"
          nuxt
          :to="localePath('/signup')"
        >
          {{ $t("page.home.start") }}
        </v-btn>

        <v-btn
          variant="text"
          class="ml-2 text-none text-grey font-weight-regular"
          size="large"
          href="https://github.com/randoooom/eventplanner"
        >
          {{ $t("page.home.github") }}
          <v-icon size="small" class="ml-1" icon="mdi-arrow-right" />
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<style lang="sass" scoped>
#title
  background: linear-gradient(90deg, rgba(140, 158, 255, 1) 0%, rgba(220, 220, 255, 1) 100%)
  background-clip: text
  -webkit-text-fill-color: transparent

.v-btn
  font-size: 16px
  border-radius: 8px

.v-card
  border-radius: 10px
</style>
