import { useEntitlements, usePermissions, useFeatureFlags } from "../hooks";

const useAccess = () => {
  const entitlements = useEntitlements();
  const permissions = usePermissions();
  const featureFlags = useFeatureFlags();

  return {
    studio:
      entitlements.studio && permissions.canAccessStudio && featureFlags.studio,
    CRM: entitlements.CRM && permissions.canAccessCRM && featureFlags.CRM,
    analytics:
      entitlements.analytics &&
      permissions.canAccessAnalytics &&
      featureFlags.analytics,
  };
};

export default useAccess;
