// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// ChangesModal is currently replaced by a link to the parsec doc history.

// describe('Check about page', () => {
//   beforeEach(() => {
//     cy.visitApp('coolorg');
//     cy.login('Boby', 'P@ssw0rd.');
//     cy.get('#profile-button').click();
//     cy.get('.popover-viewport').find('.version').click();
//   });

//   afterEach(() => {
//     cy.dropTestbed();
//   });

//   it('Opens the changes modal', () => {
//     cy.get('.info-list').find('.changelog-btn').contains('Show changelog').click();
//     cy.get('.changes-modal').get('.title-h2').contains('Changes');
//     cy.get('.changes-modal')
//       .find('.version-title')
//       .contains(/Parsec Cloud v[\da-z.-]+/);
//     cy.get('.changes-modal').find('.version').should('have.length.above', 0);
//   });

//   it('Close when click X', () => {
//     cy.get('.changes-modal').should('not.exist');
//     cy.get('.info-list').find('.changelog-btn').contains('Show changelog').click();
//     cy.get('.changes-modal').should('exist');
//     cy.get('.closeBtn').click();
//     cy.get('.changes-modal').should('not.exist');
//   });
// });
