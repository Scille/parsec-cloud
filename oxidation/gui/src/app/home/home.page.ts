import { Component } from '@angular/core';
import { ToastController } from '@ionic/angular';


@Component({
  selector: 'app-home',
  templateUrl: 'home.page.html',
  styleUrls: ['home.page.scss'],
})
export class HomePage {
  key = "1UT2bs6chdW4AnXbkSS18EuwOAgWIr7ROcHnicUhdAA=";
  message = "Ym9ueW91ciE=";

  constructor(private toastController: ToastController) {}

  async onSubmit(): Promise<any> {
    console.log("calling encrypt...");
    const encrypted = await this.encrypt(this.key, this.message);
    console.log(encrypted);
    console.log("calling decrypt...");
    const decrypted = await this.decrypt(this.key, encrypted);
    console.log(decrypted);
    this.toastController.create({
      message: decrypted,
      duration: 2000
    }).then((toast) => {
      toast.present();
    });
  }

  async encrypt(key: string, cleartext: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const w: any = window;
      w.libparsec.submitJob(resolve, reject, "encrypt", `${key}:${cleartext}`);
    });
  }

  async decrypt(key: string, cyphertext: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const w: any = window;
      w.libparsec.submitJob(resolve, reject, "decrypt", `${key}:${cyphertext}`);
    });
  }
}
