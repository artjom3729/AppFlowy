import { usePublishContext } from '@/application/publish';
import Outline from '@/components/publish/outline/Outline';
import { Divider, PopperPlacementType } from '@mui/material';
import React, { ReactElement, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import RichTooltip from 'src/components/_shared/popover/RichTooltip';
import { ReactComponent as Logo } from '@/assets/logo.svg';
import { ReactComponent as AppflowyLogo } from '@/assets/appflowy.svg';

export function OutlinePopover({
  children,
  open,
  onClose,
  placement,
  onMouseEnter,
  onMouseLeave,
}: {
  open: boolean;
  onClose: () => void;
  children: ReactElement;
  placement?: PopperPlacementType;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}) {
  const viewMeta = usePublishContext()?.viewMeta;
  const { t } = useTranslation();

  const content = useMemo(() => {
    return (
      <div
        onMouseEnter={onMouseEnter}
        onMouseLeave={onMouseLeave}
        className={'flex h-fit max-h-[500px] w-[268px] flex-col overflow-y-auto p-2'}
      >
        <Outline viewMeta={viewMeta} />
        <div
          style={{
            position: 'sticky',
            bottom: 0,
            width: '100%',
            height: '44px',
          }}
          className={'flex flex-col items-center justify-center gap-3 bg-bg-body'}
        >
          {Boolean(viewMeta?.child_views?.length) && <Divider className={'w-full'} />}

          <div className={'flex w-full items-center justify-center gap-4 text-sm'}>
            <div className={'text-text-caption'}>{t('publish.createdWith')}</div>
            <div
              className={'flex cursor-pointer items-center justify-center text-text-title'}
              onClick={() => {
                window.open('https://appflowy.io', '_blank');
              }}
            >
              <Logo className={'h-4 w-4'} />
              <AppflowyLogo className={'w-20'} />
            </div>
          </div>
        </div>
      </div>
    );
  }, [onMouseEnter, onMouseLeave, t, viewMeta]);

  return (
    <RichTooltip open={open} onClose={onClose} content={content} placement={placement}>
      {children}
    </RichTooltip>
  );
}

export default OutlinePopover;
