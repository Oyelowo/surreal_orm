// Don't Edit. This is autogenerated.
export interface ISealedSecretsBitnami {
	global: Global;
	kubeVersion: string;
	nameOverride: string;
	fullnameOverride: string;
	namespaceOverride: string;
	commonLabels: CommonLabels;
	commonAnnotations: CommonLabels;
	clusterDomain: string;
	extraDeploy: any[];
	image: Image;
	command: any[];
	commandArgs: any[];
	args: any[];
	containerPorts: ContainerPorts;
	resources: Resources;
	livenessProbe: LivenessProbe;
	readinessProbe: LivenessProbe;
	startupProbe: LivenessProbe;
	customLivenessProbe: CommonLabels;
	customReadinessProbe: CommonLabels;
	customStartupProbe: CommonLabels;
	podSecurityContext: PodSecurityContext;
	containerSecurityContext: ContainerSecurityContext;
	hostAliases: any[];
	podLabels: CommonLabels;
	podAnnotations: CommonLabels;
	podAffinityPreset: string;
	podAntiAffinityPreset: string;
	nodeAffinityPreset: NodeAffinityPreset;
	affinity: CommonLabels;
	nodeSelector: CommonLabels;
	tolerations: any[];
	updateStrategy: UpdateStrategy;
	priorityClassName: string;
	topologySpreadConstraints: any[];
	schedulerName: string;
	terminationGracePeriodSeconds: string;
	lifecycleHooks: CommonLabels;
	extraEnvVars: any[];
	extraEnvVarsCM: string;
	extraEnvVarsSecret: string;
	extraVolumes: any[];
	extraVolumeMounts: any[];
	sidecars: any[];
	initContainers: any[];
	service: Service;
	ingress: Ingress;
	rbac: Rbac;
	serviceAccount: ServiceAccount;
	networkPolicy: NetworkPolicy;
	metrics: Metrics;
}
interface Metrics {
	serviceMonitor: ServiceMonitor;
}
interface ServiceMonitor {
	enabled: boolean;
	namespace: string;
	labels: CommonLabels;
	annotations: CommonLabels;
	jobLabel: string;
	honorLabels: boolean;
	interval: string;
	scrapeTimeout: string;
	metricRelabelings: any[];
	relabelings: any[];
	selector: CommonLabels;
}
interface NetworkPolicy {
	enabled: boolean;
	allowExternal: boolean;
}
interface ServiceAccount {
	create: boolean;
	name: string;
	annotations: CommonLabels;
	automountServiceAccountToken: boolean;
}
interface Rbac {
	create: boolean;
	pspEnabled: boolean;
	unsealer: Unsealer;
	keyAdmin: Unsealer;
	serviceProxier: Unsealer;
}
interface Unsealer {
	rules: any[];
}
interface Ingress {
	enabled: boolean;
	pathType: string;
	apiVersion: string;
	ingressClassName: string;
	hostname: string;
	path: string;
	annotations: CommonLabels;
	tls: boolean;
	selfSigned: boolean;
	extraHosts: any[];
	extraPaths: any[];
	extraTls: any[];
	secrets: any[];
	extraRules: any[];
}
interface Service {
	type: string;
	ports: ContainerPorts;
	nodePorts: NodePorts;
	clusterIP: string;
	loadBalancerIP: string;
	loadBalancerSourceRanges: any[];
	externalTrafficPolicy: string;
	annotations: CommonLabels;
	extraPorts: any[];
	sessionAffinity: string;
	sessionAffinityConfig: CommonLabels;
}
interface NodePorts {
	http: string;
}
interface UpdateStrategy {
	type: string;
}
interface NodeAffinityPreset {
	type: string;
	key: string;
	values: any[];
}
interface ContainerSecurityContext {
	enabled: boolean;
	readOnlyRootFilesystem: boolean;
	runAsNonRoot: boolean;
	runAsUser: number;
}
interface PodSecurityContext {
	enabled: boolean;
	fsGroup: number;
}
interface LivenessProbe {
	enabled: boolean;
	initialDelaySeconds: number;
	periodSeconds: number;
	timeoutSeconds: number;
	failureThreshold: number;
	successThreshold: number;
}
interface Resources {
	limits: CommonLabels;
	requests: CommonLabels;
}
interface ContainerPorts {
	http: number;
}
interface Image {
	registry: string;
	repository: string;
	tag: string;
	digest: string;
	pullPolicy: string;
	pullSecrets: any[];
	debug: boolean;
}
interface CommonLabels {}
interface Global {
	imageRegistry: string;
	imagePullSecrets: any[];
	storageClass: string;
}
