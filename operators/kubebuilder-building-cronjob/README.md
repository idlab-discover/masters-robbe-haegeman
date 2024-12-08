# Building Cronjob - Kubebuilder
When attempting to learn Kube.rs, I found that the tutorials were often quite lacking when compared to Kubebuilder.
This is quite logical, since Kubebuilder is both more mature and more widely used.
It however also sparked the idea to follow a Kubebuilder tutorial ([Kubebuilder book: Building Cronjob](https://book.kubebuilder.io/cronjob-tutorial/cronjob-tutorial)) using Kube.rs.
This would allow me to compare some of the differences, while also improving my Kube.rs and Rust skills.

This was originally done in a private repo, but after it sparked some conversation, it seems more appropriate to make it public.
This is normally just an exact replica of the code described in the tutorial, without tweaks and such.
I'm not claiming this as my code, all rights go to the developers behind it.
It is however a lot easier to compare the Kube.rs and Kubebuilder solutions if they are in the same repo.

## Commands used in the tutorial

### 1. Tutorial: Building Cronjob
```sh
kubebuilder init --domain tutorial.kubebuilder.io --repo tutorial.kubebuilder.io/project
```
###  1.4 Adding a new API
```sh
kubebuilder create api --group batch --version v1 --kind CronJob
```

###  1.8 Implementing defaulting/validating webhooks
```sh
kubebuilder create webhook --group batch --version v1 --kind CronJob --defaulting --programmatic-validation
```

### 1.9 Running and deploying the controller
```sh
make manifests
make install
```
```sh
kubectl create -f config/samples/batch_v1_cronjob.yaml
kubectl get cronjob.batch.tutorial.kubebuilder.io -o yaml
kubectl get job
```
