import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";
import { environmentVariables } from "./validations";

// import { devNamespace } from "./namespaces";

// import * as eks from "@pulumi/eks";

// export const namespace = new k8s.Name;

// const nameSpaceName = "development";
const { ENVIRONMENT, TEMPORARY_DIR } = environmentVariables;

//  I am first putting all resources in a single cluster and allocating resources and envronment based on namespace rather than cluster.
// i.e  type Namespace = "development" | "staging" | "production". And only a single cluster.

// If need be, in the future, we can have three providers(clusters):
// type Cluster = "development" | "staging" | "production".
// while namespace can then be used for categorising resources based on logical grouping or team allocation. e.g
// type Namespace = "team-a" | "workers" | "web" | "jobs"

// TODO: probably use path and __dirname modules?
const rootDirectory = `manifests/generated/${TEMPORARY_DIR ?? ENVIRONMENT}`;
export const applicationsDirectory = new k8s.Provider("render-applications", {
  renderYamlToDirectory: `${rootDirectory}/applications`,
  // renderYamlToDirectory: `${rootDirectory}/${nameSpaceName}`,
  // namespace: "nana",
});

export const nameSpacesDirectory = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: `${rootDirectory}/namespaces`,
  // namespace: "nana",
});

// Stores resources useful for starting a fresh cluster such as the
// sealed secrets controller and ingress controller which are
// fundamental for the applications that would run in the cluster
export const clusterSetupDirectory = new k8s.Provider("render-cluster-setup", {
  renderYamlToDirectory: `${rootDirectory}/cluster-setup`,
  // namespace: "nana",
});

export const argocdDirectory = new k8s.Provider("render-argocd", {
  renderYamlToDirectory: `${rootDirectory}/argocd`,
  // namespace: "nana",
});

// export const devNamespace = new k8s.core.v1.Namespace(
//   "local",
//   {
//     metadata: { name: nameSpaceName, namespace: nameSpaceName },
//   },
//   { provider }
// );
