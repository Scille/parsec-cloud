import { alertController } from '@ionic/vue';

export async function createAlert(
  header: string,
  message: string,
  cancelLabel: string,
  OkLabel: string
): Promise<HTMLIonAlertElement> {
  return await alertController.create({
    header: header,
    message: message,
    buttons: [
      {
        text: cancelLabel,
        role: 'cancel'
      },
      {
        text: OkLabel,
        role: 'confirm'
      }
    ]
  });
}
