import { config } from 'dotenv';
import { Application, Router } from 'oak';
import { PrismaClient } from 'prisma';

const env = await config();

const prisma = new PrismaClient({
  datasources: {
    db: {
      url: env.db_url,
    },
  },
});
const app = new Application();
const router = new Router();

/**
 * Setup routes.
 */

router
  .get('/', context => {
    context.response.body = 'Welcome to the Dinosaur API!';
  })
  .get('/dinosaur', async context => {
    // Get all dinosaurs.
    const dinosaurs = await prisma.dinosaur.findMany();
    context.response.body = dinosaurs;
  })
  .get('/dinosaur/:id', async context => {
    // Get one dinosaur by id.
    const { id } = context.params;
    const dinosaur = await prisma.dinosaur.findUnique({
      where: {
        id: Number(id),
      },
    });
    context.response.body = dinosaur;
  })
  .post('/dinosaur', async context => {
    // Create a new dinosaur.
    const { name, description } = await context.request.body('json').value;
    const result = await prisma.dinosaur.create({
      data: {
        name,
        description,
      },
    });
    context.response.body = result;
  })
  .delete('/dinosaur/:id', async context => {
    // Delete a dinosaur by id.
    const { id } = context.params;
    const dinosaur = await prisma.dinosaur.delete({
      where: {
        id: Number(id),
      },
    });
    context.response.body = dinosaur;
  });

/**
 * Setup middleware.
 */

app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 8000 });
