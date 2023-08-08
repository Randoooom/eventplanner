/*
 *     Copyright (C) 2023 Fritz Ochsmann
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published
 *     by the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import {
  defineNuxtRouteMiddleware,
  useNuxtApp,
  navigateTo,
  useLocalePath,
} from "#imports";

export default defineNuxtRouteMiddleware((to) => {
  const { $surrealdb } = useNuxtApp();
  const localePath = useLocalePath();

  // if the client is not authenticated redirect to /
  if (!$surrealdb.loggedIn.value || $surrealdb.scope.value !== "account")
    return navigateTo(localePath("/"));
});
