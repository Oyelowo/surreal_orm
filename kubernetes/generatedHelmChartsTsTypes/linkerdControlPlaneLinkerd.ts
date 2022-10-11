// Don't Edit. This is autogenerated.
export interface ILinkerdControlPlaneLinkerd {
    clusterDomain: string;
    clusterNetworks: string;
    imagePullPolicy: string;
    controllerLogLevel: string;
    controllerLogFormat: string;
    controlPlaneTracing: boolean;
    controlPlaneTracingNamespace: string;
    linkerdVersion: string;
    deploymentStrategy: DeploymentStrategy;
    enableEndpointSlices: boolean;
    enablePodAntiAffinity: boolean;
    enablePprof: boolean;
    enablePodDisruptionBudget: boolean;
    cniEnabled: boolean;
    identityTrustAnchorsPEM: string;
    identityTrustDomain: string;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    priorityClassName: string;
    runtimeClassName: string;
    policyController: PolicyController;
    proxy: Proxy;
    proxyInit: ProxyInit;
    imagePullSecrets: any[];
    enableH2Upgrade: boolean;
    enablePSP: boolean;
    webhookFailurePolicy: string;
    controllerImage: string;
    controllerReplicas: number;
    controllerUID: number;
    debugContainer: DebugContainer;
    identity: Identity;
    disableHeartBeat: boolean;
    proxyInjector: ProxyInjector;
    profileValidator: ProfileValidator;
    policyValidator: ProfileValidator;
    nodeSelector: NodeSelector;
}
interface NodeSelector {
    'kubernetes.io/os': string;
}
interface ProfileValidator {
    externalSecret: boolean;
    namespaceSelector: NamespaceSelector;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
    injectCaFrom: string;
    injectCaFromSecret: string;
}
interface ProxyInjector {
    externalSecret: boolean;
    namespaceSelector: NamespaceSelector;
    objectSelector: ObjectSelector;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
    injectCaFrom: string;
    injectCaFromSecret: string;
}
interface ObjectSelector {
    matchExpressions: MatchExpression2[];
}
interface MatchExpression2 {
    key: string;
    operator: string;
}
interface NamespaceSelector {
    matchExpressions: MatchExpression[];
}
interface MatchExpression {
    key: string;
    operator: string;
    values: string[];
}
interface Identity {
    externalCA: boolean;
    serviceAccountTokenProjection: boolean;
    issuer: Issuer;
}
interface Issuer {
    scheme: string;
    clockSkewAllowance: string;
    issuanceLifetime: string;
    tls: Tls;
}
interface Tls {
    crtPEM: string;
    keyPEM: string;
}
interface DebugContainer {
    image: Image;
}
interface ProxyInit {
    iptablesMode: string;
    ignoreInboundPorts: string;
    ignoreOutboundPorts: string;
    skipSubnets: string;
    logLevel: string;
    logFormat: string;
    image: Image;
    resources: Resources;
    closeWaitTimeoutSecs: number;
    runAsRoot: boolean;
    runAsUser: number;
    xtMountPath: XtMountPath;
}
interface XtMountPath {
    mountPath: string;
    name: string;
}
interface Proxy {
    enableExternalProfiles: boolean;
    outboundConnectTimeout: string;
    inboundConnectTimeout: string;
    image: Image;
    logLevel: string;
    logFormat: string;
    ports: Ports;
    cores: number;
    resources: Resources;
    uid: number;
    waitBeforeExitSeconds: number;
    await: boolean;
    requireIdentityOnInboundPorts: string;
    opaquePorts: string;
    shutdownGracePeriod: string;
    defaultInboundPolicy: string;
}
interface Ports {
    admin: number;
    control: number;
    inbound: number;
    outbound: number;
}
interface PolicyController {
    image: Image;
    logLevel: string;
    probeNetworks: string[];
    resources: Resources;
}
interface Resources {
    cpu: Cpu;
    memory: Cpu;
    'ephemeral-storage': Cpu;
}
interface Cpu {
    limit: string;
    request: string;
}
interface Image {
    name: string;
    pullPolicy: string;
    version: string;
}
interface PodAnnotations {}
interface DeploymentStrategy {
    rollingUpdate: RollingUpdate;
}
interface RollingUpdate {
    maxUnavailable: string;
    maxSurge: string;
}
