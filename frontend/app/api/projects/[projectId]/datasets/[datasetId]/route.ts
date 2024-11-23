import { and, eq } from 'drizzle-orm';
import { authOptions } from '@/lib/auth';
import { datasets } from '@/lib/db/migrations/schema';
import { db } from '@/lib/db/drizzle';
import { fetcher } from '@/lib/utils';
import { getServerSession } from 'next-auth';

export async function GET(
  req: Request,
  { params }: { params: { projectId: string; datasetId: string } }
): Promise<Response> {
  const projectId = params.projectId;
  const datasetId = params.datasetId;

  const dataset = await db.query.datasets.findFirst({
    where: and(eq(datasets.id, datasetId), eq(datasets.projectId, projectId))
  });

  if (!dataset) {
    return new Response('Dataset not found', { status: 404 });
  }

  return new Response(JSON.stringify(dataset), { status: 200 });
}

export async function DELETE(
  req: Request,
  { params }: { params: { projectId: string; datasetId: string } }
): Promise<Response> {
  const projectId = params.projectId;
  const datasetId = params.datasetId;
  const session = await getServerSession(authOptions);
  const user = session!.user;

  const res = await fetcher(`/projects/${projectId}/datasets/${datasetId}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${user.apiKey}`
    }
  });

  return new Response(res.body);
}
