import { ReactComponent as BoardSvg } from '$icons/16x/board.svg';
import { ReactComponent as CalendarSvg } from '$icons/16x/date.svg';
import { ReactComponent as DocumentSvg } from '$icons/16x/document.svg';
import { ReactComponent as GridSvg } from '$icons/16x/grid.svg';
import { ViewLayout } from '@/application/collab.type';
import { usePublishContext } from '@/application/publish';
import { notify } from '@/components/_shared/notify';
import React from 'react';
import { useTranslation } from 'react-i18next';

const renderCrumbIcon = (icon: string) => {
  if (Number(icon) === ViewLayout.Grid) {
    return <GridSvg className={'h-4 w-4'} />;
  }

  if (Number(icon) === ViewLayout.Board) {
    return <BoardSvg className={'h-4 w-4'} />;
  }

  if (Number(icon) === ViewLayout.Calendar) {
    return <CalendarSvg className={'h-4 w-4'} />;
  }

  if (Number(icon) === ViewLayout.Document) {
    return <DocumentSvg className={'h-4 w-4'} />;
  }

  return icon;
};

export interface Crumb {
  viewId: string;
  rowId?: string;
  name: string;
  icon: string;
}

function BreadcrumbItem({ crumb, disableClick = false }: { crumb: Crumb; disableClick?: boolean }) {
  const { viewId, icon, name } = crumb;

  const { t } = useTranslation();
  const onNavigateToView = usePublishContext()?.toView;

  return (
    <div
      className={`flex items-center gap-1 text-sm ${!disableClick ? 'cursor-pointer' : 'flex-1 overflow-hidden'}`}
      onClick={async () => {
        if (disableClick) return;
        try {
          await onNavigateToView?.(viewId);
        } catch (e) {
          notify.error(t('publish.hasNotBeenPublished'));
        }
      }}
    >
      <span className={'icon'}>{renderCrumbIcon(icon)}</span>
      <span
        className={!disableClick ? 'max-w-[250px] truncate hover:text-text-title hover:underline' : 'flex-1 truncate'}
      >
        {name || t('menuAppHeader.defaultNewPageName')}
      </span>
    </div>
  );
}

export default BreadcrumbItem;