// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

// HelloError
export interface HelloErrorEmptySubject {
    tag: 'EmptySubject'
}
export interface HelloErrorYouAreADog {
    tag: 'YouAreADog'
    hello: string;
}
export type HelloError =
  | HelloErrorEmptySubject
  | HelloErrorYouAreADog

export interface LibParsecPlugin {
    helloWorld(subject: string): Promise<Result<string, HelloError>>;
}
