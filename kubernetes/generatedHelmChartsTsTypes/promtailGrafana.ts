// Don't Edit. This is autogenerated.
export interface IPromtailGrafana {
	nameOverride?: any;
	fullnameOverride?: any;
	daemonset: Daemonset;
	deployment: Deployment;
	initContainer: any[];
	image: Image;
	imagePullSecrets: any[];
	annotations: Annotations;
	updateStrategy: Annotations;
	podLabels: Annotations;
	podAnnotations: Annotations;
	priorityClassName?: any;
	livenessProbe: Annotations;
	readinessProbe: ReadinessProbe;
	resources: Annotations;
	podSecurityContext: PodSecurityContext;
	containerSecurityContext: ContainerSecurityContext;
	rbac: Rbac;
	serviceAccount: ServiceAccount;
	nodeSelector: Annotations;
	affinity: Annotations;
	tolerations: Toleration[];
	defaultVolumes: DefaultVolume[];
	defaultVolumeMounts: DefaultVolumeMount[];
	extraVolumes: any[];
	extraVolumeMounts: any[];
	extraArgs: any[];
	extraEnv: any[];
	extraEnvFrom: any[];
	serviceMonitor: ServiceMonitor;
	extraContainers: Annotations;
	extraPorts: Annotations;
	podSecurityPolicy: PodSecurityPolicy;
	config: Config;
	networkPolicy: NetworkPolicy;
	extraObjects: any[];
}
interface NetworkPolicy {
	enabled: boolean;
	metrics: Metrics;
	k8sApi: K8sApi;
}
interface K8sApi {
	port: number;
	cidrs: any[];
}
interface Metrics {
	podSelector: Annotations;
	namespaceSelector: Annotations;
	cidrs: any[];
}
interface Config {
	logLevel: string;
	serverPort: number;
	clients: Client[];
	snippets: Snippets;
	file: string;
}
interface Snippets {
	pipelineStages: PipelineStage[];
	common: Common[];
	addScrapeJobLabel: boolean;
	extraLimitsConfig: string;
	extraServerConfigs: string;
	extraScrapeConfigs: string;
	extraRelabelConfigs: any[];
	scrapeConfigs: string;
}
interface Common {
	action: string;
	source_labels: string[];
	target_label: string;
	replacement?: string;
	separator?: string;
	regex?: string;
}
interface PipelineStage {
	cri: Annotations;
}
interface Client {
	url: string;
}
interface PodSecurityPolicy {
	privileged: boolean;
	allowPrivilegeEscalation: boolean;
	volumes: string[];
	hostNetwork: boolean;
	hostIPC: boolean;
	hostPID: boolean;
	runAsUser: RunAsUser;
	seLinux: RunAsUser;
	supplementalGroups: RunAsUser;
	fsGroup: RunAsUser;
	readOnlyRootFilesystem: boolean;
	requiredDropCapabilities: string[];
}
interface RunAsUser {
	rule: string;
}
interface ServiceMonitor {
	enabled: boolean;
	namespace?: any;
	namespaceSelector: Annotations;
	annotations: Annotations;
	labels: Annotations;
	interval?: any;
	scrapeTimeout?: any;
	relabelings: any[];
	metricRelabelings: any[];
}
interface DefaultVolumeMount {
	name: string;
	mountPath: string;
	readOnly?: boolean;
}
interface DefaultVolume {
	name: string;
	hostPath: HostPath;
}
interface HostPath {
	path: string;
}
interface Toleration {
	key: string;
	operator: string;
	effect: string;
}
interface ServiceAccount {
	create: boolean;
	name?: any;
	imagePullSecrets: any[];
	annotations: Annotations;
}
interface Rbac {
	create: boolean;
	pspEnabled: boolean;
}
interface ContainerSecurityContext {
	readOnlyRootFilesystem: boolean;
	capabilities: Capabilities;
	allowPrivilegeEscalation: boolean;
}
interface Capabilities {
	drop: string[];
}
interface PodSecurityContext {
	runAsUser: number;
	runAsGroup: number;
}
interface ReadinessProbe {
	failureThreshold: number;
	httpGet: HttpGet;
	initialDelaySeconds: number;
	periodSeconds: number;
	successThreshold: number;
	timeoutSeconds: number;
}
interface HttpGet {
	path: string;
	port: string;
}
interface Annotations {}
interface Image {
	registry: string;
	repository: string;
	tag?: any;
	pullPolicy: string;
}
interface Deployment {
	enabled: boolean;
	replicaCount: number;
	autoscaling: Autoscaling;
}
interface Autoscaling {
	enabled: boolean;
	minReplicas: number;
	maxReplicas: number;
	targetCPUUtilizationPercentage: number;
	targetMemoryUtilizationPercentage?: any;
}
interface Daemonset {
	enabled: boolean;
}
