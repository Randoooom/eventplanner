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

import { defineNuxtPlugin } from "#imports";
// @ts-ignore
import { Surreal } from "surrealdb.wasm/http";
import { Ref, ref } from "#imports";

type Scope = "account" | "visitor";

/**
 *
 */
export class SurrealdbConnection {
  public connection: Surreal;
  public loggedIn: Ref<boolean>;
  public scope: Ref<Scope | undefined>;

  /**
   *
   */
  constructor() {
    this.connection = new Surreal();
    this.loggedIn = ref(false);
    this.scope = ref(undefined);
  }

  /**
   *
   * @param surrealdbEndpoint
   */
  async init(surrealdbEndpoint: string) {
    await this.connection.connect(surrealdbEndpoint);
  }

  /**
   *
   * @param username
   * @param password
   */
  async signup(username: string, password: string) {
    const token = await this.connection.signup({
      username,
      password,
      namespace: "production",
      database: "eventplanner",
      scope: "account",
    });
    await this.connection.authenticate(token);

    this.scope.value = "account";
    this.loggedIn.value = true;
  }

  /**
   *
   * @param username
   * @param password
   */
  async login(username: string, password: string) {
    const token = await this.connection.signin({
      username,
      password,
      namespace: "production",
      database: "eventplanner",
      scope: "account",
    });
    await this.connection.authenticate(token);

    this.scope.value = "account";
    this.loggedIn.value = true;
  }

  /**
   *
   * @param id
   */
  async visit(id: string) {
    const token = await this.connection.signin({
      id,
      namespace: "production",
      database: "eventplanner",
      scope: "visitor",
    });
    await this.connection.authenticate(token);
    this.scope.value = "visitor";
    this.loggedIn.value = true;
  }

  /**
   *
   */
  async logout() {
    await this.connection.invalidate();
    this.loggedIn.value = false;
  }
}

export default defineNuxtPlugin(async (app) => {
  const connection = new SurrealdbConnection();

  // connect to the database
  await connection.connection.connect(app.$config.public.surrealdbEndpoint);

  return {
    provide: {
      surrealdb: connection,
    },
  };
});
